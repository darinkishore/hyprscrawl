# Dusk integration

Upstream: ShamanicArts/hyprscrawl. Deploy branch: `vendor`.

Nix packaging, runtime dependencies, launcher entries and the session-bound
systemd service are owned by `~/Documents/nix-config/den/vendor/hyprscrawl.nix`
and `pkgs/hyprscrawl/`. Do not run `scrawl setup service` or `setup launcher`
over that declarative installation.

Linux paste uses a clipboard copy plus Hyprland `hl.dsp.send_shortcut`,
addressed to the original focused window. It refuses a focus change before
either the image or summary paste. No virtual keyboard or character streaming.
The doctor check no longer requires wtype.

## Timed transcription

`transcriber: "voxtype"` calls `voxtype transcribe --json --file-model
 gpt-transcribe --timestamps`. Voxtype owns the API credentials, vocabulary,
file transcription and local WhisperX acoustic alignment. The adapter requires
complete, monotonic word timings and never falls back to estimated timings.
Failure keeps the recording, labels the bundle `voxtype:failed`, and notifies.

The Nix installation owns `~/.config/scrawl/config.json` and its systemd unit.
Change those defaults in nix-config rather than running upstream setup commands.
