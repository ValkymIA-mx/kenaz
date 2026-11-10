# M4: Beyond accounts (W8–W11, W13)

**Goal:** the tasks that appear on the home screen once the first account is protected. Three independent sub-tracks, each staffed across KEY, VAULT, CORE, UI, CONTENT and SHEET.

**Depends on:** M2 (secrets file, checklist, both-keys write flow). Runs in parallel with M3. D3 needed by M4c, D4 by M4b.

**Shared by all three:** screens built from the wireframes drawn in M2; the "shown once, copy it by hand" screen for admin codes; adding their entries to the restore set and the reset safety check (see [M3 hand-off](phase-3-recovery.md)); sheet sections for their data.

**DESIGN, across the sub-tracks:** the home screen grows from W1–W5 to the full task list once the first account is protected, without pushing "Protect an account" and the codes out of sight; one guide layout for every third-party app (KeePassXC, Okular, Thunderbird, sign-in) with illustrations per D15; run U3 and fix what stops a participant.

## M4a: Button features (W8, W9)

- [ ] KEY: read OTP slot config; swap short/long press; delete the factory credential; write HMAC-SHA1 challenge-response to a slot.
- [ ] VAULT: challenge-response secret generated in memory, written to both keys, stored in the file.
- [ ] CORE/UI: W8 explanation and the two fixes (long press recommended; removal warns it can't be put back); W9 password manager picker → write to both keys → steps.
- [ ] CONTENT: KeePassXC guide first; computer login with challenge-response.
- [ ] SHEET: challenge-response secrets section.

## M4b: Certificates and e.firma (W10, W13)

- [ ] VAULT: P-256 key + self-signed certificate in memory (D4); e.firma: parse `.key` (password-protected PKCS#8) and `.cer`, check they match, read name, RFC and expiry.
- [ ] KEY: import key + certificate to 9a/9c on both keys; change PUK from factory; PIN-protected random management key (`piv_util`); test signature after every e.firma import.
- [ ] CORE/UI: W10 (create → both keys → Linux sign-in pairing per D10 → "more uses": SSH, PDF signing); W13 with the three blocks: expired e.firma, factory PUK (offer to change now), spare without PIV (warn).
- [ ] W5 status shows the e.firma expiry date.
- [ ] CONTENT: Linux sign-in (D10), SSH, Okular PDF signing guides; e.firma copy in Spanish first. The Windows sign-in guide comes in M5 (D11).
- [ ] SHEET: PIV section (management key, certificates, e.firma origin).
- [ ] PIV unlock option for the secrets file, if not already shipped in M2 (D2).

## M4c: Encrypt and sign email (W11)

- [ ] VAULT: OpenPGP Curve25519 key pair + certificate in memory (D3); admin PIN and reset code generated.
- [ ] KEY: import to both keys; set admin PIN, reset code, touch policy; on-card decryption for the unlock option.
- [ ] CORE/UI: W11 (name + email → key → both keys → export public key → mail app steps); touch setting.
- [ ] OpenPGP unlock option for the secrets file, offered only on firmware 5.2.3+ (not on YubiKey 4; the cut-off itself is untested).
- [ ] CONTENT: Thunderbird first, then Kleopatra and GnuPG.
- [ ] SHEET: OpenPGP fingerprints section.

## Exit criteria

- Each workflow passes on the YubiKey 5 and, where it applies, with the Security Key as spare. YubiKey 4 and pre-5.7 paths are capability-gated but untested (see [hardware matrix](overview.md#hardware-test-matrix)).
- Restore (W6) rebuilds challenge-response, PIV, e.firma and OpenPGP onto a new key from the secrets file.
- e.firma: test signature verified on each key; Okular finds the certificate (or an upstream issue is filed).
- U3 done: every issue that stopped a participant on W9–W11 and W13 is fixed and re-checked.
