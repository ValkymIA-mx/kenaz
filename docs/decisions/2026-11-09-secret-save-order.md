# When a new secret is saved to the secrets file

**Date:** 2026-11-09
**Status:** accepted

## Decision

- **A new secret is saved to the encrypted secrets file right after the first key's write succeeds**, before the app asks for the second key. This covers 6-digit code secrets, OpenPGP and PIV keys, and challenge-response secrets.
- Order for every flow that puts a secret on both keys: create or receive the secret in memory → write it to the first key → save it to the secrets file → write it to the second key (now, or later through W4) → wipe it from memory.
- The unlock method for the secrets file is chosen before the first write, so the file exists when the save is due.

## Why

Three orders were considered:

| Order | Rejected or chosen because |
|---|---|
| Save before writing any key | Nothing is lost once setup starts, but the file can hold a secret that is on no key if the write fails, and CORE has to clean those up. Rejected. |
| **Save after the first key's write succeeds** | The file only holds secrets that are really on a key. Loss is limited to two short windows: from the site showing the secret until the first write finishes, and from that write until the save commits. Chosen. |
| Save after both keys | Contradicts W4: a skipped spare could never be finished from the file. It also leaves the longest window where closing the app loses the secret. Rejected. |

## Consequences

- `IDEA.md` → Backup (product policy) and W2 state the order above.
- The key write and the save are two separate operations and can't be made atomic together. **W4 recovery is guaranteed only once the save has committed:** the new file is fully written and has replaced the old one, so a crash mid-save leaves the previous file intact. Until then (including a crash after the first key's write succeeded but before the save committed, or a failed save), the secret exists only in memory and on the first key; if the app closes then, the spare can't be finished from the file. For a code account the site's setup has to be repeated; for OpenPGP, PIV or challenge-response the key pair or secret has to be made again and set up again in the apps that use it.
- On restart, the app finds such a secret by comparing what is on the key with the file: an entry on the key with no saved secret is shown as "on this key only, the spare needs a new setup", never as "spare missing" that W4 could finish.
- If the save fails, the app keeps the secret in memory and says so in plain words. It asks for the spare right away and retries the save; it never reports the account as protected on both keys while the secret is saved nowhere.
- The in-memory fallback (no unlock method works on the user's keys, so there is no secrets file) has no save step: the spare must be written before the app closes, and the recovery sheet is the only copy.
- CORE records, per secret, which keys hold it and whether the save has committed, so the UI can say exactly what an interruption cost ([M2](../roadmap/phase-2-accounts.md)).
