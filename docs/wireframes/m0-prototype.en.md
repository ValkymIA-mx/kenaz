# M0 — Paper prototype (English)

**Draft for U0; wording and navigation unvalidated.** This is not a working application or an M1 decision. Print or copy each H1–H10 section as a card. The facilitator changes cards according to the indicated route; action names are printed text, not clickable buttons. Use paper tokens for keys and separate browser and printer cards. Bracketed numbers are facilitator references, not product text. “Main” and “spare” are provisional names, not a decision about distinguishing the keys (D17).

## Screen map

- Home H1 → first setup H2–H4 (W1); choose account H5 → H6 → H7 or H8 → H9–H10 (W2). H1 also leads to codes, status, checklist, replacement and details in H11. Return to Home is available in every section.
- H11 → checklist → finish spare: sign-in sites return to H6–H7; saved secrets are finished from the file. H11 → replace key → W6 journey below. The checklist shows what can be detected and allows manual entries for what cannot.
- After the first account is protected, Home offers later tasks W8–W13; before then they do not appear. W6 and W7 are available as recovery, not later tasks. This map does not settle layout, style or illustrations (D13–D16 pending).

## H1 — Home

**What do you want to protect?**

Start with your email: it helps you recover your other accounts. You choose which account to protect.

- Set up my keys → H2 (if not yet set up)
- Protect an account → H5
- Get a 6-digit code → H11, Codes
- Am I protected? → H11, Status
- See my checklist and finish my spare → H11, Checklist
- I lost or broke a key → H11, Replacement
- I forgot my PIN or it is blocked → H11, PIN
- Advanced details → H11, Details

If there is no spare: **You do not have a spare key yet. If you lose this key, you could lose access.** → Set up spare / Continue without a spare.

Only after the first account is protected: “My key types letters”, “Unlock my password manager”, “Sign in to my computer”, “Encrypt and sign email”, “Give away or erase a key”, and “Sign PDFs with my e.firma” → W8–W13; not developed in this prototype.

## H2 — Your first key (W1)

Plug in your main YubiKey. We will check it before continuing.

Waiting for a key… → if recognised, H3. If not a YubiKey: “This is not a supported YubiKey. Plug in a YubiKey.” If another program is using it: “Another program is using your key. Close it and try again.” If access or service is missing: “We cannot access the key. Check the connection and permissions or the key service.” → Try again / Back to Home.

## H3 — Choose your PIN

Choose a PIN of 6 to 8 characters: unaccented letters, numbers or ASCII symbols. Do not use ñ or accented letters. Enter it again to confirm. Do not share it.

If invalid or mismatched: “Check your PIN and enter it again.” If the key has additional rules, explain them before saving. If fewer than the maximum tries remain: “[n] tries remain; if they run out, your key may be blocked.” → Continue / Back.

**Facilitator:** D8 remains open: this card does not promise when factory PIV/OpenPGP PINs or administrative codes are changed. Do not create the secrets file here.

## H4 — Set up the spare

Plug in your second YubiKey to use the same PIN. If you have only one port, unplug the main key and plug in the spare. If asked, touch the indicated key. Wait until it finishes; do not unplug while it saves.

- Second key ready → H5.
- I do not have a spare → “You can continue, but if you lose your key you could lose access. Some things cannot be copied later.” → Continue without a spare → H5 / Plug in spare.
- Wrong key, busy key or unplugged during operation → “We did not finish setting up this key. Plug in the indicated key and try again.” Do not say it is ready → Try again / Back to Home.

## H5 — Choose an account (W2)

What account do you want to protect? We suggest email first, then banking, then social media. You can choose another or enter its name.

Email / Banking / Social media / Enter a site → Continue. If the site is not listed: “We will show general steps; the site may call them something else.” → H6.

## H6 — On the site

Open [site]'s security settings in your browser. Follow that site's steps; Kenaz does not register the key for you.

1. Sign in to [site].
2. Open Security and find ways to sign in.
3. Follow the option below. Name it “YubiKey main”.
4. If the browser asks, plug in and touch the main key. Return here when the site confirms it was added.

The route is fixed by the site: first sign in without a password using your key; if unsupported, use your key after your password; if neither is supported, 6-digit codes → H8. Sign-in with key → H7. Do not pretend the site confirmed.

**Space:** on the first passkey, “Your keys can hold up to [lower capacity] sign-ins like this. Many sites do not use this space.” If there are two keys and one older than 5.7 already holds 20 passkeys, before each site: “[lowest free space] spaces left; used by [sites].” Show that an absent key's reading is from [date]. The person may use a space or choose key after password if the site offers it; nothing is removed automatically. If the browser says the key is full: “There is no room. Review your sign-ins; before removing one, make sure you have another way to sign in.” → Checklist / Back.

## H7 — Now add the spare

On the same site, add another key and name it “YubiKey spare”. With one port, unplug the main key and plug in the spare; if asked, touch the spare. Return when the site confirms it was added. Each key is registered separately.

- The site confirmed both → H9.
- Not now / I have no spare → “This account still has no spare. If you lose the main key, you could lose access.” → H9 with “spare missing” on the checklist.
- The site fails or the key is unplugged → “The spare was not confirmed.” → Try again / Leave pending → H9 with “spare missing”.

## H8 — 6-digit codes

On the site, open code setup. Capture the on-screen QR code or paste the setup link. The first time, choose one or more available ways to open your encrypted secrets file; any one chosen way opens it. If none works with your keys: “We cannot save a file your keys can open. To use a spare, you must finish both keys before closing the app.” → Continue in memory / Cancel.

Plug in the main key; touch it if asked. “Saving to the main key… Do not unplug it.” After writing it: “Saving to the encrypted file…” Only after saving is confirmed, plug in the spare (unplug the main key first if there is one port), touch it if asked and wait until finished. If there is no spare or it fails after saving: “The spare is pending; you can finish it using the encrypted file.” → H9 pending. If the app closes before the file save is confirmed: “You cannot finish the spare from the file; you must set up again.” The secret is never saved unencrypted.

Without a file, if using a spare both keys need room and the spare must be written before closing; if interrupted, do not promise recovery from a file. Continuing with only the main key is allowed, with a warning that there will be no saved copy for finishing the spare later. If the main key is full: “There is no room; nothing was written or saved.” If the spare is full, warn before writing when known: “You can make room or continue with the spare pending.” With a file, write the main key and save the file; the checklist says “spare missing: no room”. Without a file and with a spare, write nothing until both have room. If the spare does not support codes: “This key does not support codes; the spare will remain pending.” Without a file, do not promise it can be finished later. → H9 pending / Cancel. After the first code account, offer the recovery sheet → H10.

## H9 — The site's backup codes

Did the site give you backup codes? Print them from the site or copy them by hand and keep them with your recovery sheet. Kenaz does not store or display them. If printing, use the browser and collect the pages from the printer.

I am done / It gave me no codes → H10. If there is only one key: “The spare is missing; losing the key could leave you without access.”

## H10 — Your checklist

[Account]: both keys / spare missing / missing on this key, according to what was confirmed. “A sign-in with a key must be added separately for each key.” Accounts that cannot be detected are added and checked off by hand.

Offer a recovery sheet when applicable: “Print one sheet per key and keep it somewhere safe. Write the private codes shown by hand; the PIN and codes that can change it are never printed.” → Print: leave for printer, collect sheets / Not now. → Home H1 / Protect another account H5 / Checklist H11.

## H11 — Other map destinations

- **Codes (W3):** “Plug in a key to see your codes.” List codes on the connected key → Copy; if requested, “Touch your key.” Empty: “There are no code accounts on this key.”
- **Status (W5):** number of sites on both keys and pending, spaces used on each key, age of last reading if absent, external copy of the encrypted file and PIN tries remaining on each key. If unknown, show “unknown”, not a made-up number.
- **Checklist (W4):** sites “both keys”, “spare missing” or “missing on this key”; add and check off sites that cannot be detected by hand. A saved secret is finished from the file; a key sign-in site requires returning to the browser and registering the spare separately. → H6 or H8 as applicable.
- **Replacement (W6):** follow the journey below. **PIN (W7):** warn about tries and consequences before any action; a sign-in reset invalidates every credential on that key. Actually open the file using a surviving method before any reset; if none opens it, stop.
- **Advanced details:** real terms: sign-in with key = passkey, key after password = security key, codes = OATH TOTP, encrypted file = age; unlock options OpenPGP, PIV or FIDO2. This does not assume a D2 recommendation.

## Journey and handoffs — W1, W2 and W6

| Moment | Person / system | Exit or state |
|---|---|---|
| W1 | Plug in main → check → choose PIN → plug in spare or unplug main and swap → touch if asked; or skip spare | H2–H4; persistent warning if missing |
| W2 sign-in | Choose site → leave for browser → plug in/touch main if asked → return → unplug/swap or plug in spare → return to browser and touch if asked → return | H5–H7; confirmation per key, or pending |
| W2 codes | Leave for browser for QR/link → return, choose file unlock if first time → plug in/touch main → write main → confirm encrypted save → swap/plug in and touch spare if asked | H8; file-free exception and errors described there |
| W2 finish | Leave for browser to print or hand-copy site codes → collect printouts; if sheet offered, leave for printer and collect one per key → return | H9–H10; checklist reflects confirmations |
| W6 1 | **First:** open email in browser and remove the lost key; then remove it from the other checklist sites. Use the surviving key or the site's recovery method to sign in; return to the app | If unable to sign in, use site recovery; do not claim removal |
| W6 2 | Plug in surviving and new keys; with one port, unplug and swap when instructed. Set the same PIN on the new key; touch when asked | If survivor is missing or PIN fails, stop and explain; do not claim restoration |
| W6 3 | Open the file with the surviving key; touch if asked. Swap/plug in the new key and write available codes, OpenPGP/PIV identities and button secrets | If the file cannot be opened, stop restoration and explain those secrets cannot be copied from the key; use available recovery |
| W6 4 | Key sign-ins **cannot** be restored from the file. Every site becomes “missing on new key”. Open each site in the browser, sign in with the survivor or another method, swap to the new key and touch if asked to register it; return and check off only after confirmation | Some sites delay use: Google can take up to 7 days. If it fails, it remains pending |
| W6 5 | If the site offers new backup codes, print or copy by hand; collect printouts. When appropriate, print and collect the new key's sheet | Checklist and sheet reflect only completed work |
