# M5: Windows port

**Goal:** every workflow (W1–W13) works on the latest Windows 11 release, x86-64 and ARM64, with the admin prompt only where FIDO access needs it.

**Depends on:** M3 and M4 done on Linux.

## KEY + PLATFORM

- [ ] Enumerate keys over PC/SC and HID on Windows; device info matches Linux.
- [ ] D6: try CTAP2 over CCID on USB; list exactly which operations need administrator rights. Codes, OpenPGP, PIV and OTP must work without admin.
- [ ] Add the "needs admin" error to `KeyPort`; run only the passkey operations and the FIDO2 unlock option elevated.
- [ ] `scdaemon` from Gpg4win: detection signal and fix on Windows.
- [ ] CI: build + test for Windows x86-64 and ARM64. Guests WIN1 and WIN2 added to the VM lab.
- [ ] Windows implementations of the OS modules: app data folder, QR from screen, process checks.

## VAULT

- [ ] Re-check D2 with D6's answer: if the FIDO2 unlock option needs the admin prompt on every open, decide whether Windows recommends a different option for new setups.

## UI + CONTENT

- [ ] Plain explanation shown before the admin prompt on passkey screens and FIDO2 unlock.
- [ ] D11: what W10 offers on Windows; write the guide or show plainly that computer sign-in isn't offered there.
- [ ] Guides checked on Windows: Thunderbird, Kleopatra (Gpg4win), GnuPG, KeePassXC, Okular or the chosen PDF signer, SSH.
- [ ] Windows look and accessibility: fonts, scaling at 125% and 150%, high-contrast themes, keyboard-only use and Narrator at the D14 level, on WIN1.

## Exit criteria

- W1–W13 pass on Windows x86-64 (WIN1) with the hardware matrix; the ARM64 build runs in WIN2.
- Files from the secrets file made on Linux open on Windows and the other way round.
- No admin prompt outside passkey screens and the FIDO2 unlock option.
- The D14 accessibility level holds on Windows as it does on Linux.
