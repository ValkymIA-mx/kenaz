# M6: Release 1.0

**Goal:** ship 1.0 on Linux and Windows, x86-64 and ARM64. macOS is not part of 1.0 (see [M7](phase-7-macos.md)).

**Depends on:** M5.

## Work

- [ ] PLATFORM: full pass of W1–W13 on the YubiKey 5 and Security Key across the VM lab (Linux guests LX1–LX5, Windows guests WIN1–WIN2), including the two-YubiKey-5 pair flows; record results in `docs/reports/`.
- [ ] PLATFORM: D9, packaging and signing for Linux and Windows (deferred until now by `IDEA.md` → Later).
- [ ] CONTENT: no untranslated strings; native review of every es and en string; site list and guides checked against the live sites.
- [ ] CONTENT: release notes list what's supported (Wayland sessions only; the latest release of each tested distro plus Ubuntu LTS; the latest Windows 11), the recorded hardware gaps (YubiKey 4, YubiKey 5 before 5.7) as untested, and that macOS isn't supported.
- [ ] VAULT/KEY: review of secret handling: nothing secret on disk unencrypted, memory wiped after use, files open outside the app with the plugins.
- [ ] DESIGN: run U4 on the release build (W1–W7, Linux and Windows); fix what stops a participant. Earlier rounds U0–U3 already shaped the screens, so this round confirms rather than discovers.
- [ ] DESIGN/UI: accessibility audit of W1–W13 against D14 (keyboard-only, Orca, Narrator, contrast, text scaling) and a visual pass at 100%, 125% and 150% in es and en.
- [ ] Re-check `yubikit`: move to the latest release or confirm the pinned version.

## Exit criteria

- Every workflow passes on the Linux guests (x86-64 and ARM64) and Windows x86-64 with the YubiKey 5 and the Security Key, including the pair flows on two YubiKey 5. A failure here blocks the release; release notes don't waive it.
- The only waivable gaps are the ones recorded in the [hardware matrix](overview.md#hardware-test-matrix) (YubiKey 4, YubiKey 5 before 5.7) and Windows ARM64 full passes if emulation in WIN2 is too slow; each is listed in the release notes.
- Signed packages for Linux and Windows per D9.
- U4 passed and the D14 accessibility audit passed on Linux and Windows x86-64. Like the workflow passes, a failure here blocks the release.
