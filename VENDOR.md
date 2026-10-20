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
