# M2: Accounts on two keys (W1–W5)

**Goal:** the first job of the product. A person with two blank keys ends with their email protected on both keys, 6-digit codes on both keys, a secrets file and a recovery sheet.

**Depends on:** M1; D2, D8, D15, D16.

## KEY

- [ ] CTAP2: set/change PIN, PIN tries left, list and delete stored passkeys, passkey spaces used and left (credential management).
- [ ] OATH: add account (TOTP, touch flag), list, calculate, delete; free space per key.
- [ ] Apply the shared PIN per D8.
- [ ] Unlock-option operations for the identities chosen in D2 (PIV P-256 key in a retired slot and/or FIDO2 `hmac-secret`).

## VAULT

- [ ] Unlock options from D2: set up on both keys, encrypt to every recipient, open with either key.
- [ ] Add a recipient later (spare added after the fact) by re-encrypting.
- [ ] OATH secret entries; parse `otpauth://` setup links (pasted or from a scanned QR).

## CORE

- [ ] W1 state machine: key 1 check → PIN → key 2 (same PIN; one-port swap) → skip with spare warning.
- [ ] W2: site → method order (passkey, security key, codes) → "now add your spare" → backup-code advice → checklist update. The first code account asks for the unlock option before anything is written to a key, then offers the sheet.
- [ ] Secret save order per the [decision](../decisions/2026-11-09-secret-save-order.md): first key → save to the secrets file → second key → wipe. A failed save keeps the secret in memory, asks for the spare right away and retries; the account is never shown as on both keys while the secret is saved nowhere.
- [ ] W3: codes from whichever key is plugged; touch-required codes flagged.
- [ ] W4: "spare missing" items; codes copied from the secrets file, passkey sites routed to W2 steps.
- [ ] W5: status (sites on both keys, spare missing, secrets file copy outside the app folder, PIN tries per key).
- [ ] Checklist from passkeys on both keys (match by site), plus manual entries for security keys and undetectable sites.
- [ ] Repeated spare warning: home screen and after every account on one key only.
- [ ] Storage rules per `IDEA.md` → Key storage limits: limits from each key's firmware; total shown as the lowest capacity across the user's keys, "spaces left" and warnings from the lowest free space, using last-known counts (stored with the non-secret app state, with when they were read) for a key that isn't plugged in; the 20-passkey warning triggered by a pre-5.7 key's own count; the "key full" path. Code space: a full first key blocks the write; a full spare never blocks the first key's write or the save, and becomes a "spare missing: no room" item for W4 (warned up front); both keys need room only in the in-memory fallback.
- [ ] W1 and W2 record what has been written to which key, and whether each secret's save has committed, so an interruption leaves a known state the UI can explain.

## UI

- [ ] Screens for W1–W5 built from the M1 wireframes, every state included; "unplug this key and plug in the spare" screen.
- [ ] PIN entry: live ASCII check with the plain explanation; tries-left warning before entry.
- [ ] Code list: one-click copy, "touch your key" state.
- [ ] Add-code paths: scan QR on screen, paste setup link.
- [ ] Open the site's security settings page; show 3–5 illustrated steps.
- [ ] "Another program is using your key" screen with the one-click fix where available; plain screens for "key service not running" and "no permission for the device" on Linux.
- [ ] Interrupted flows: a key pulled mid-write or the app closed mid-W1/W2 ends in a known state, with a plain message about what was written, what wasn't, and what the user has to redo. Once a secret's save has committed, nothing is lost: the missing key is finished through W4 from the secrets file. A secret whose save never committed (closed before or during the save, a failed save, or the in-memory fallback when no unlock option works) means repeating the site's setup for the spare.
- [ ] W2 window layout per D16, checked side by side with Firefox and Chrome at 1366×768 and with Spanish text.

## DESIGN

- [ ] Step illustrations for the curated site list v1, per D15, in es and en.
- [ ] Recovery sheet print flow: printer dialog, "shown once, copy it by hand" boxes, and the reminder when the sheet is due.
- [ ] Spare warning: one wording and placement that stays visible without turning into noise the user learns to ignore (home screen, after each one-key account).
- [ ] Wireframes for M3 (W6, W7, W12) and M4 (W8–W11, W13), every state included, reviewed before those screens are built.
- [ ] Run U1 on the built app; fix what stops a participant.

## CONTENT

- [ ] Curated site list v1, es and en, with illustrated steps; generic steps for unlisted sites.
- [ ] All copy for W1–W5, including the spare warning, the unlock-option explanations, the passkey storage disclosure and warning, and the suggested site order (email, banking, social media).

## SHEET

- [ ] Build sheet data from app state (OATH secrets + QR, passkey sites, recipients, unlock method material such as the PIV management key if D2 picks PIV).
- [ ] Mark the sheet as due whenever one of its secrets changes.

## Exit criteria

On real keys, in es and en, on Linux:

- Fresh pair → email protected with a passkey on both keys → one code account on both keys → secrets file created → sheet printed.
- Spare skipped at W1, then finished through W4 with the codes restored from the secrets file.
- App closed after a code account's save to the secrets file has committed, before the second key is written: on restart the account shows "spare missing", and W4 finishes it from the file without repeating the site's setup.
- App killed after the first key's write succeeded but before the save committed (forced with a test hook between the two operations): on restart the previous secrets file opens intact, the account shows "on this key only, the spare needs a new setup" (not "spare missing"), and W4 does not offer to copy it.
- A file written by the app opens outside it with `age`/`rage` + the matching plugin.
- U1 done: every issue that stopped a participant on W1–W3 is fixed and re-checked.
