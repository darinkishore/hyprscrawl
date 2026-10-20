//! Timed file transcription delegated to Voxtype. It owns credentials, context,
//! the file API and acoustic alignment. No fallback to synthetic word times.
use crate::model::{TranscriptData, WordTiming};
use anyhow::{Context, Result, ensure};
use serde::Deserialize;
use std::path::Path;
use std::time::Duration;

pub fn enabled() -> bool {
    !crate::transcribe::overridden()
        && std::env::var("SCRAWL_TRANSCRIBER")
            .ok()
            .or_else(crate::handoff::transcriber)
            .is_some_and(|name| name.trim().eq_ignore_ascii_case("voxtype"))
}

#[derive(Deserialize)]
struct Word {
    text: String,
    start: f64,
    end: f64,
    score: f64,
}
#[derive(Deserialize)]
struct Reply {
    text: String,
    model: String,
    timing: Option<String>,
    words: Vec<Word>,
}

fn parse(bytes: &[u8]) -> Result<TranscriptData> {
    let reply: Reply =
        serde_json::from_slice(bytes).context("Voxtype returned invalid timed JSON")?;
    ensure!(
        reply.timing.as_deref() == Some("whisperx:wav2vec2-base-960h"),
        "Voxtype did not return acoustic alignment"
    );
    ensure!(
        reply
            .text
            .split_whitespace()
            .eq(reply.words.iter().map(|w| w.text.as_str())),
        "Voxtype alignment omitted transcript words"
    );
    let mut previous = 0.0;
    let mut words = Vec::new();
    for word in reply.words {
        ensure!(
            word.start.is_finite()
                && word.end.is_finite()
                && word.score.is_finite()
                && word.start >= previous
                && word.end > word.start
                && (0.0..=1.0).contains(&word.score),
            "Voxtype returned invalid acoustic word timings"
        );
        previous = word.end;
        words.push(WordTiming {
            text: word.text,
            t0: (word.start * 1000.0).round() as u64,
            t1: (word.end * 1000.0).round() as u64,
        });
    }
    Ok(TranscriptData {
        text: reply.text,
        source: format!("voxtype:{}+whisperx", reply.model),
        model: Some(reply.model),
        estimated: false,
        segments: crate::linux::chatgpt::segments_of(&words),
        words,
    })
}

pub fn transcribe(audio: &Path) -> Result<TranscriptData> {
    let bytes = crate::helper::output(
        "voxtype",
        &[
            "transcribe".as_ref(),
            audio.as_os_str(),
            "--json".as_ref(),
            "--file-model".as_ref(),
            "gpt-transcribe".as_ref(),
            "--timestamps".as_ref(),
        ],
        Duration::from_secs(1800),
    )?;
    parse(&bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn aligned_word_times_survive_conversion() {
        let data = parse(br#"{"text":"Look here","model":"gpt-transcribe","timing":"whisperx:wav2vec2-base-960h","words":[{"text":"Look","start":1.2,"end":1.5,"score":0.9},{"text":"here","start":1.6,"end":2.0,"score":0.8}]}"#).unwrap();
        assert!(!data.estimated);
        assert_eq!(data.words[0].t0, 1200);
        assert_eq!(data.words[1].t1, 2000);
    }
    #[test]
    fn rejects_missing_alignment_instead_of_spreading_words() {
        assert!(
            parse(br#"{"text":"Look here","model":"gpt-transcribe","timing":null,"words":[]}"#)
                .is_err()
        );
        assert!(parse(br#"{"text":"Look here","model":"gpt-transcribe","timing":"whisperx:wav2vec2-base-960h","words":[]}"#).is_err());
    }
}
