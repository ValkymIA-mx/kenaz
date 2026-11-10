# M3: Recovery and reset (W6, W7, W12)

**Goal:** the high-stakes paths. Losing a key, blocking a PIN or resetting a key never silently locks anyone out. With M2 this completes the product core (W1–W7).

**Depends on:** M2. Runs in parallel with M4.

**Hand-off with M4:** the reset safety check is built here against the M2 unlock options. When M4b/M4c add PIV keys, OpenPGP keys and the OpenPGP unlock option, the check and W6 restore must cover them; each M4 sub-track owns adding its cases.

## KEY

- [ ] CTAP2 reset, including the replug-within-seconds window the key enforces (exact timing to verify on hardware).
- [ ] Full device reset per applet.
- [ ] PIV unblock with PUK; OpenPGP unblock with reset code (exercised once M4 lands, built here).
- [ ] Tries-left reads for every PIN the app manages.

## VAULT

- [ ] Re-encrypt to drop a dead recipient and add a new one.
- [ ] Restore set: everything needed to rebuild a key from the file (codes now; challenge-response, PIV, OpenPGP added by M4).

## CORE

- [ ] Reset safety check: work out which unlock methods survive the planned reset, actually open the file with one, then continue. If none opens it: stop with the plain warning ("after this reset, nobody can ever open your secrets file again, and the spare can't be rebuilt from it"), offer the ways out first (plug in the spare, add another unlock method while this key still works), and allow the reset only after an explicit confirmation of that consequence.
- [ ] W6: remove-from-email first → remaining key + new key → same PIN → restore from the secrets file → every passkey site marked "missing on new key".
- [ ] W7: tries-left warning before every PIN entry with fewer than all tries; PIV/OpenPGP unblock from the spare's secrets file; passkey PIN reset path with the full consequence explanation and affected sites marked "missing on this key".
- [ ] W12: what will be lost → remove from every site first → safety check → final confirmation → reset.

## UI

- [ ] Screens for W6, W7, W12 built from the wireframes drawn in M2; the reset confirmation; the "nobody can ever open your secrets file again" warning screen with the ways out and a separate explicit confirmation to reset anyway.
- [ ] "Can't be undone" pattern from the M1 component inventory on every reset: the destructive button is never the default or focused one, and the ways out come first.
- [ ] CTAP2 reset screen: guides the replug within the key's time window (countdown, touch prompt), and says plainly what to do if the window is missed.

## DESIGN

- [ ] Run U2: participants read the W6, W7 and W12 warnings and say in their own words what will be lost. Rewrite with CONTENT any warning someone gets wrong.

## CONTENT

- [ ] Copy for W6, W7, W12, reviewed as high-stakes text (each warning names the consequence in plain words).
- [ ] Delay warning for sites that hold back a new key (Google: up to 7 days).

## SHEET

- [ ] New sheet for the replacement key; prompt to destroy the old one.

## Exit criteria

Scripted drills on real keys:

- Lose key A, replace with key C: codes restored, checklist shows every passkey site as missing on C, file re-encrypted without A.
- Block the passkey PIN on one key while a surviving method exists: the reset proceeds only after the file opens with it; affected sites marked.
- Attempt a reset that would orphan the secrets file: the app stops and offers the ways out; taking a way out (plug in the spare) makes the check pass.
- Same orphaning reset, declining every way out: nothing is reset until the explicit confirmation; after it, the reset runs.
- U2 done: every participant explains the consequence of each reset warning correctly after the rewrites.
