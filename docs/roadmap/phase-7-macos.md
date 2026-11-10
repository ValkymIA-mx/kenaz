# M7: macOS port (needs a contributor)

**Status:** not scheduled. Starts only if an external contributor with Mac hardware (and ideally a YubiKey 5 and a Security Key) takes it on. Nobody on the project can build or test on macOS.

**Goal:** every workflow (W1–W13) works on macOS, ARM64 and x86-64, including macOS account pairing for W10.

**Depends on:** 1.0 (M6). The OS-specific modules from M1 are the extension points.

## KEY + PLATFORM

- [ ] Enumerate keys over PC/SC and HID on macOS; device info matches Linux.
- [ ] `scdaemon` from GPG Suite / Homebrew GnuPG: detection signal and fix on macOS.
- [ ] CI: build + test for macOS ARM64 and x86-64.
- [ ] macOS implementations of the OS modules: app data folder, QR from screen (screen recording permission), process checks.

## VAULT

- [ ] The self-signed 9a certificate (D4) is accepted by macOS account pairing.

## UI + CONTENT

- [ ] Plain explanation shown before the screen recording permission prompt.
- [ ] W10 macOS account pairing guide.
- [ ] Guides checked on macOS: Thunderbird, GnuPG, KeePassXC, the chosen PDF signer, SSH.
- [ ] Keyboard-only use and VoiceOver at the D14 level; macOS window and menu conventions where they differ from Linux and Windows.
- [ ] Packaging and signing for macOS (extends D9).

## Exit criteria

- W1–W13 pass on macOS with the contributor's keys.
- Files from the secrets file made on Linux or Windows open on macOS and the other way round.
- A Mac account pairs with the key through the W10 guide.
