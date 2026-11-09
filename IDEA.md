# yubicox

A simple desktop app that helps everyday people — the ones still using the same password everywhere — protect their accounts with a YubiKey, and keep a spare so losing it isn't a disaster. Once the accounts are safe, the same key can also sign and encrypt email and files, sign in to a computer, and unlock a password manager.

## Problem

Current tools assume you already know what you're doing:

- **Kleopatra** is powerful, but you need to understand public vs. secret keys, certificates, smartcards, and trust before you can do anything useful.
- **Yubico Authenticator** uses friendly labels (Accounts, Passkeys, Fingerprints, Certificates, Slots), but each section is a separate feature. Nothing tells you where to start, which accounts to protect first, or how to avoid getting locked out if you lose the key.

Neither tool helps someone go from "I reuse one password" to "my important accounts are protected and I have a backup".

## Who it's for

- People who reuse passwords and have never set up two-step login.
- People who were handed a YubiKey and don't know what to do with it.
- People whose accounts are already protected and want the next step: signing or encrypting email and files, signing in to a computer or server with the key, or unlocking a password manager with it.
- Not for: security engineers, organisations rolling out keys and certificates to staff, or anyone who needs custom OpenPGP key layouts. Yubico Authenticator, Kleopatra and `ykman` already cover them.

## Positioning

Not a replacement for Yubico Authenticator. It's a **guided companion** for the first and most common job: securing everyday accounts. Email/file encryption (OpenPGP), certificates and computer sign-in (PIV) and button features (OTP slots) come after that job, as tasks on the same home screen, never as the starting point. Anyone who outgrows it can switch to the official tools with the same key.

## Scope

- **YubiKey only**, on desktop (macOS, Linux, Windows; x86-64 and ARM64). Android and iOS are on the roadmap, not in the initial release.
- **Passkeys and security keys (FIDO2/U2F):** explain what they are, guide the user through adding the key on popular sites, either as a passkey (sign in with the key, no password) or as a security key (the key as a second step after the password). Manage the key's PIN, and list or remove passkeys stored on the key.
- **6-digit codes (OATH TOTP):** add accounts by scanning a QR code on screen or pasting a setup link; show codes; copy them with one click.
- **Encrypt and sign (OpenPGP):** create one key pair for the person, put it on both YubiKeys, export the public key, and guide the user through using it in common apps (Thunderbird, Kleopatra, GnuPG). Manage the PIN, admin PIN and reset code, and the "touch to use" setting. The app does not send or read email itself.
- **Certificates and smart card (PIV):** create one key pair and a certificate for each use (sign in, sign documents), put them on both YubiKeys, and export the certificate. Guide the user through common uses such as pairing the key with their Mac account, SSH, and signing PDFs. Import a certificate someone else issued. Manage the PIN and PUK; the management key stays out of sight (see Encrypted secrets file).
- **Button features (OTP slots):** explain why the key types random letters when touched (factory Yubico OTP in the short-press slot) and offer to move it to long press or remove it. Set up challenge-response for password managers and computer login (for example KeePassXC), on both keys.
- **Encrypted secrets file (age):** every secret the app needs to set up a spare or replacement key later (6-digit code setup secrets, challenge-response secrets, the OpenPGP and PIV keys, the OpenPGP admin PIN and reset code, the PIV PUK) is kept in one file encrypted with [age](https://age-encryption.org/v1) to the YubiKeys. Nothing secret is ever written to disk unencrypted.
- **Backup key:** setup asks for two keys from the start and treats the second key as part of setup, not an extra. The spare is optional, but skipping it brings a clear, repeated warning (see User workflows).

## Principles

1. **Tasks, not features.** The home screen asks "What do you want to protect?", not "Choose an application".
2. **Plain language first.** No jargon on the main path. An "advanced details" view shows the real terms for the curious.
3. **The backup is part of setup.** Every flow that puts something on one key (an account, an OpenPGP or PIV key, a button secret) also puts it on the spare. If the user skips the spare, they get a clear, repeated warning.
4. **Never lock someone out silently.** Warn before PIN retry limits run out, before a reset, and before deleting anything. Explain the consequences in plain words.
5. **Start with what matters most.** Suggest protecting email first (it's the key to resetting every other account), then banking, then social media.

## Backup: hard limits vs. product policy

Secrets can't be copied off a YubiKey. What that means depends on the feature:

- **Passkeys and security keys:** each website registers each key separately. You can add a spare key to a site later, as long as you can still sign in (with the first key or another method). Yubico recommends registering the spare straight away, but doesn't require it ([source](https://www.yubico.com/products/spare/)). The app can't register keys on websites itself; it can only guide the user and keep a checklist of which sites have both keys. Security keys used as a second step aren't stored on the YubiKey at all; the site keeps them.
- **6-digit codes:** the spare needs the same setup secret (the QR code or setup link) as the first key, and that secret can't be read back off the first key. To add the spare later, the user needs a saved copy of the secret, or has to turn two-step login off and on again on that site, which makes the first key's codes stop working.
- **OpenPGP and PIV:** a key pair generated on a YubiKey can never leave it, so two keys that each generate their own pair end up as two different identities: other people (or servers) would need both public keys, a certificate issued for one key doesn't work on the other, and losing one key means losing what was encrypted only to it. Having the same key pair on both YubiKeys means generating it outside the keys and writing it to each. Certificates are public and can be copied freely.
- **OTP challenge-response:** same as 6-digit codes. The secret can't be read back, so both keys need it written at setup, or the password manager has to be re-enrolled later.

**Product policy:** set up both keys together for every feature. That's the only clean option for codes, OpenPGP, PIV and challenge-response, and it stops passkey spares from being forgotten. The app should:

- 6-digit codes, OpenPGP, PIV and challenge-response: write the secret to both keys in one step if both are plugged in, otherwise prompt for the second key while the secret is still in memory. Then add it to the encrypted secrets file (below) and wipe it from memory. Never write it to disk unencrypted.
- OpenPGP and PIV: generate key pairs in the app's memory (not on the YubiKey), so both YubiKeys hold the same identity. This trades "the key never existed outside the hardware" for "losing one YubiKey loses nothing", which is the right trade for most people.
- 6-digit codes: as an opt-in, also offer a printable recovery sheet with the QR code, for the case where both YubiKeys are lost and the secrets file can't be opened. The sheet should explain plainly that anyone holding it can make that site's codes, and suggest storing it like a passport.
- Passkeys and security keys: prompt "now add your spare key" right after each site is registered, and keep any site with only one key on the checklist as "spare missing" so the user can finish it later.

## Encrypted secrets file (age)

One file, in the standard [age](https://age-encryption.org/v1) format, holds the secrets listed in Scope. It lets the user replace a lost key: plug in the surviving key, unlock the file, and the app writes the 6-digit code accounts, the OpenPGP and PIV keys and the challenge-response secrets onto the new key.

**Passkeys and security keys can't be restored this way.** They aren't in the file and can't be copied (see Backup), so the user has to register the replacement key separately on every website, signing in with the surviving key. The app adds every such site to the checklist as "missing on this key" until that's done, and says this plainly before the replacement starts.

### What opens the file: the user chooses

The first time the app needs to store a secret (the first 6-digit code account, OpenPGP or PIV key, or challenge-response secret), the user picks one or more ways to unlock the file. Setting up the chosen method happens then, not at first-time setup. age encrypts to every chosen method, and any one of them opens it. Each option is explained in plain words; which one to recommend is still open.

| Option | Opens with | Works on | Open outside the app with | Trade-offs |
|---|---|---|---|---|
| **OpenPGP** (Curve25519 decryption key) | the OpenPGP key that's already on both YubiKeys | YubiKey 5, firmware 5.2.3+ ([Yubico](https://www.yubico.com/blog/whats-new-in-yubikey-firmware-5-2-3/)) | `age`/`rage` + [`age-plugin-openpgp-card`](https://github.com/wiktor-k/age-plugin-openpgp-card); the recipient is a plain `age1…` key, so encrypting needs no plugin | Requires setting up OpenPGP, even for people who never sign an email. Small, little-known plugin. |
| **PIV** (P-256 key in a retired slot) | a PIV key on both YubiKeys | YubiKey 4 and 5 | `age`/`rage` + [`age-plugin-yubikey`](https://github.com/str4d/age-plugin-yubikey) | The most widely used and packaged YubiKey age plugin. Takes one PIV slot. |
| **FIDO2** (`hmac-secret`) | the passkey part of each YubiKey; one recipient per key | Any FIDO2 key, including the Security Key series | `age`/`rage` + [`age-plugin-fido2-hmac`](https://github.com/olastor/age-plugin-fido2-hmac) | No extra applet, uses the passkey PIN. On Windows, opening the file triggers the admin (UAC) prompt (see Platform & tech). A replacement key needs a new recipient, so the file is re-encrypted. |

Whatever the user picks gets a plain name ("your secrets key"); "OpenPGP", "PIV" and "FIDO2" only appear in the advanced details view. If none of the options works on the user's keys, the app falls back to the in-memory flow and the printable sheet, and says so.

### PINs

- **One PIN for the person:** the passkey PIN, the OpenPGP user PIN and the PIV PIN are set to the same value. It must be 6 to 8 **ASCII** characters (letters without accents, digits, common symbols): PIV stores the PIN as 6 to 8 bytes and the YubiKey allows any ASCII character there ([Yubico](https://docs.yubico.com/yesdk/users-manual/application-piv/pin-puk-mgmt-key.html)), while an accented letter takes 2 bytes and isn't ASCII. 6 is also the OpenPGP minimum. Because a Spanish keyboard makes "ñ" or "á" easy to type, the PIN screen checks this as the user types and explains the rule in plain words ("letters without accents, numbers and symbols"), instead of failing later when the PIN is written to the key.
- **Admin codes are different and random:** the OpenPGP admin PIN, the OpenPGP reset code and the PIV PUK are never the same as the PIN. The app generates them from printable ASCII (PUKs with bytes above `0x7F` are rejected from firmware 5.7) and stores them only in the secrets file; the user never has to remember them. If one key's PIN gets blocked, the spare opens the file and the app unblocks the first key with the stored reset code or PUK.
- **PIV management key:** random, and stored on the key protected by the PIN, which is how `ykman` and Yubico Authenticator handle it (`yubikit`'s `piv_util` covers this). Other tools keep working with the key.

### Other rules

- **Where it lives:** the app's data folder, plus a prompt to keep a copy somewhere else (cloud drive, USB stick). The copy is safe to store anywhere, because it is useless without one of the YubiKeys and its PIN.
- **Not a vault:** it only holds secrets the app created or received during setup. It is not a general place for passwords or files (see Out of scope; encrypting the user's own files is on the roadmap).
- **Check before any reset.** A reset destroys that key's way of opening the file: a passkey reset kills its FIDO2 unlock credential (the reset invalidates every credential on the key, see workflow 7), and a full reset also wipes its OpenPGP and PIV keys. Before any reset, the app works out which unlock methods survive it and **actually opens the file with one of them** (the spare, or an OpenPGP/PIV unlock on the same key when only the passkey part is reset). Only then does it continue, and afterwards it re-encrypts the file to drop the dead method and add the reset key back once it's set up again. If no surviving method opens the file, the app stops and says plainly: "after this reset, nobody can ever open your secrets file again, and the spare can't be rebuilt from it." It offers the ways out first (plug in the spare, or add another unlock method while this key still works) and allows the reset only after an explicit confirmation.

## Sign-in checklist

The app reads the passkeys stored on both keys and builds the checklist from them, marking each site as "both keys", "spare missing" or "missing on this key". Security keys used as a second step, and sites that don't store a passkey on the key, can't be detected; the user adds them by hand and ticks them off. The step text asks the user to name each key on the site ("YubiKey main", "YubiKey spare"), so a lost key is easy to find and remove later.

## User workflows

Kept as common and simple as possible. Workflows 1–7 are the product; 8–12 appear on the home screen once the first account is protected.

| # | Workflow | How often |
|---|---|---|
| 1 | First-time setup | once |
| 2 | Protect an account | a few times in the first week, then rarely |
| 3 | Get a 6-digit code | daily |
| 4 | Finish the spare | occasional |
| 5 | "Am I protected?" | occasional |
| 6 | Replace a lost or broken key | rare, high stakes |
| 7 | Forgot PIN / PIN blocked | rare, high stakes |
| 8 | "My key types random letters" | rare, very common confusion |
| 9 | Unlock a password manager | once per setup |
| 10 | Sign in to my computer | once per setup |
| 11 | Encrypt and sign email | once per setup |
| 12 | Give away or reset a key | rare |

### Decisions

- **Spare at first-time setup:** optional, with a clear warning repeated on the home screen and after every account added to one key only.
- **Method per site:** passkey first, then security key, then 6-digit codes. One fixed order, no choice for the user. If the site also hands out its own backup codes, the app strongly advises "print them or copy them by hand" and keep them with the recovery sheet. The app never stores or displays them.
- **Secrets file, OpenPGP and PIV:** set up when first needed, never at first-time setup.
- **Site list:** our own curated list of popular sites per language (Spanish and English). A community directory for search is on the roadmap.
- **Daily codes:** in the main window. No tray or quick panel in the first release.

### 1. First-time setup

1. Plug in key 1. The app checks it's a YubiKey and what it supports.
2. Choose one PIN (6–8 ASCII characters, see PINs).
3. Plug in key 2; the same PIN is applied. One USB port: "unplug this key and plug in the spare" screen. Skipping is allowed, with the spare warning.
4. Straight to "What do you want to protect?", with email suggested first.

### 2. Protect an account

```mermaid
flowchart TD
    D[What do you want to protect?<br/>pick from list or type a site] --> E{Site accepts the key<br/>as a passkey?}
    E -- yes --> P[Use your key to sign in<br/>no password]
    E -- no --> S{Site accepts the key<br/>as a second step?}
    S -- yes --> K[Use your key after your password]
    S -- no --> T[6-digit codes:<br/>scan QR or paste link,<br/>write to both keys + secrets file]
    P --> SP[Now add your spare key]
    K --> SP
    SP --> B[Site gave backup codes?<br/>print or copy by hand]
    T --> B
    B --> I[Checklist updated]
```

- The app opens the site's security settings page and shows 3–5 illustrated steps from the curated list. Sites not on the list get generic steps.
- Passkey and security key are one user-facing path, "use your key"; the app says which variant the site uses.
- The first 6-digit code account also asks how to unlock the secrets file (once), then offers the printable recovery sheet.

### 3. Get a 6-digit code

The main window lists the codes from whichever key is plugged in. One click copies a code; codes that need a touch say "touch your key". Works the same with either key.

### 4. Finish the spare

The home screen shows "spare missing" items. For each one: 6-digit codes, OpenPGP, PIV and challenge-response are copied from the secrets file automatically; passkey and security key sites get the same guided steps as workflow 2.

### 5. "Am I protected?"

One status view: how many sites are on both keys, which are missing the spare, whether the secrets file has a copy outside the app's folder, and PIN tries left on each key.

### 6. Replace a lost or broken key

1. First step, always: "remove the lost key from your email account", then the other sites on the checklist.
2. Plug in the remaining key and the new one; set the same PIN on the new key.
3. Unlock the secrets file; codes, OpenPGP and PIV keys and challenge-response secrets are written to the new key.
4. Every passkey and security key site goes on the checklist as "missing on new key", with the workflow 2 steps. The app warns that some sites delay a new key: Google can take up to 7 days before a newly added security key works at sign-in ([Google](https://support.google.com/accounts/answer/6103523)).

### 7. Forgot PIN / PIN blocked

- The app warns while tries remain, in plain words, before every PIN entry with fewer than all tries left.
- OpenPGP and PIV blocked: plug in the other key, open the secrets file, unblock with the stored reset code or PUK, set a new PIN.
- Passkey PIN blocked: the only way out is a reset of the passkey part of the key. That erases every passkey on it and also stops it working as a security key on every site, because a reset invalidates all credentials, including those created as a second step ([CTAP 2.1 §6.6](https://fidoalliance.org/specs/fido-v2.1-ps-20210615/fido-client-to-authenticator-protocol-v2.1-ps-errata-20220621.html)). It also kills this key's FIDO2 unlock for the secrets file, so the reset only starts after the secrets file check (see Encrypted secrets file → Other rules). The app explains all of this before the reset and then puts every affected site on the checklist as "missing on this key".

### 8. "My key types random letters"

Explain what happened (the touch triggers the factory one-time password) and offer two fixes: move it to a long press (recommended), or remove it, with a warning that it can't be put back as it was.

### 9. Unlock a password manager

Pick the password manager from a short list (KeePassXC first) → the app writes a challenge-response secret to both keys and the secrets file → steps for enabling the key in that app.

### 10. Sign in to my computer

PIV key and certificate created in memory, written to both keys and the secrets file → guided pairing for the user's OS (macOS account pairing first) → SSH and PDF signing offered as "more uses".

### 11. Encrypt and sign email

Name and email → OpenPGP key pair created in memory, written to both keys and the secrets file → public key exported → steps for the user's mail app (Thunderbird first).

### 12. Give away or reset a key

Shows what will be lost (sites, codes, keys), asks the user to remove the key from every site on the checklist first, runs the secrets file check (see Encrypted secrets file → Other rules), then resets with a final, plain confirmation.

## Languages

Spanish and English at launch. Every string goes through the translation system from day one, with no hard-coded text.

## Platform & tech

**Initial release:** desktop on macOS, Linux and Windows (both x86-64 and ARM64). **Roadmap:** Android, then iOS. No web version: browsers can't talk to the key directly for passkey management or 6-digit codes, so a WASM build has nothing to offer.

**UI: Rust + Slint (GPLv3).**

- Mobile is covered by the same UI stack. Android has been officially supported since 1.5. iOS works for Rust apps through the winit backend and Skia renderer, and builds through Xcode on macOS (via Xcodegen and a cargo build script) ([Slint iOS guide](https://docs.slint.dev/latest/docs/slint/guide/platforms/mobile/ios/)). iOS is Rust-only, which suits us.
- Translations are built in (`@tr` with gettext), which covers Spanish and English.
- Native rendering with no web view, so the app stays small and starts fast. The look can be fully custom, which a plain-language, task-first design needs.
- Rejected:
  - **Dioxus:** desktop runs in a web view (WebView2, WKWebView, WebKitGTK), so the app looks and behaves differently on each OS. Its Android support is also web-view based.
  - **Iced/egui:** Iced has no official Android support. egui's immediate-mode style looks like a developer tool, which is the opposite of this product. Both would cost more polish work for this audience.

**Talking to the key: Yubico's official Rust `yubikit` crate (decided)** ([docs.rs](https://docs.rs/yubikit), Apache-2.0, from the `Yubico/yubikey-manager` repo). It sits behind one small module of our own, so changes in the beta API only touch that module.

- **Covers our whole scope:** OATH (6-digit codes), CTAP2 (passkeys, PIN, credential management, `hmac-secret`), OpenPGP (key import, PINs, touch policy, on-card decryption), PIV (keys, certificates, PIN/PUK, PIN-protected management key via `piv_util`), YubiOTP (slot configuration and challenge-response) and device info/management, all in one library.
- **Desktop transports are built in:** PC/SC and HID on Linux, macOS and Windows.
- **Risk: it's a prerelease.** `1.0.0-beta.0` was published 2026-11-03, a week after `1.0.0-alpha.0`. Pin the exact version and expect API changes before 1.0. Fallback if it stalls: `ctap-hid-fido2` for passkeys plus our own OATH, OpenPGP, PIV and OTP commands over `pcsc`.
- **Windows:** direct FIDO HID access needs administrator rights on Windows 10/11 ([Yubico support](https://support.yubico.com/hc/en-us/articles/360016648939)). Only the passkey screens and the FIDO2 unlock option for the secrets file should request elevation; 6-digit codes, OpenPGP, PIV and OTP slots all go over PC/SC (`yubikit` runs YubiOTP over the smart-card interface too) and must work without admin. The UAC prompt needs a plain-language explanation shown before it appears. During Windows testing, check whether CTAP2 over the smart-card (CCID) interface works over USB; if it does, we can avoid the admin prompt.
- **Risk: GnuPG holds the key.** Once the user has used the key with GnuPG, its `scdaemon` can keep exclusive access to the smart-card interface, and other apps fail to connect. The app must detect this and explain it in plain words ("another program is using your key"), with a one-click fix where the OS allows it.

**Secrets file: the `age` crate** (from the `rage` project, MIT/Apache-2.0) for the file format. Each unlock option needs an age identity that does its key step on the YubiKey through `yubikit`: X25519 key agreement with OpenPGP `decrypt`, P-256 key agreement with PIV, or `hmac-secret` with CTAP2. The PIV and FIDO2 options must write the same file format as `age-plugin-yubikey` and `age-plugin-fido2-hmac`, or the "open outside the app" promise breaks. Check early that the `age` crate's public API allows custom identities; the fallback is to run the existing plugins through age's plugin protocol.

**OpenPGP certificates:** building the public key certificate (name, email, self-signatures) needs an OpenPGP library. Candidates: `sequoia-openpgp` (LGPL-2.0-or-later) and `pgp`/rPGP (MIT/Apache-2.0); both fit a GPLv3 app. Pick during prototyping.

**PIV certificates:** self-signed X.509 certificates need a certificate library, for example `rcgen` or RustCrypto's `x509-cert`. Pick during prototyping.

## Roadmap (after the initial release)

- **Android:** `yubikit` lists no Android transport. With `default-features = false` it accepts custom connection implementations. Open question: implement its `Connection` trait over Android's USB Host and NFC APIs (via JNI), or bridge to Yubico's `yubikit-android` (Java/Kotlin) and give up the shared Rust protocol layer. Also decide whether to support phone + key over NFC, since this audience mostly lives on their phones.
- **iOS:** the UI side is covered by Slint (see above); builds need macOS + Xcode. Open question: how to reach the key on iOS (NFC and Lightning/USB-C), since `yubikit` lists no iOS transport. Options are a custom `Connection` over iOS APIs, or bridging to Yubico's iOS SDK.
- **Encrypt your own files with age:** let the user encrypt and open their own files with the same unlock options as the secrets file. This needs its own design (file picker, sharing with other people, what happens when a key is replaced), so it stays out of the first release.
- **Community site directory:** add a community-maintained directory of sites and their supported methods for search, alongside the curated list. Check the licence and data format first.

## Out of scope

- Fingerprint (Bio) management.
- OTP slot modes beyond the two tasks above: static passwords (the key would type a fixed password, the opposite of what this app teaches), programming new Yubico OTP credentials, and HOTP in slots (6-digit codes cover it).
- OpenPGP key layouts other than "one identity on both keys": offline primary keys, per-key subkeys, RSA, SSH via `gpg-agent`.
- Using the secrets file as a general vault for passwords, notes or files.
- Other key vendors (Nitrokey, SoloKey, …).
- Password manager features and fixing password reuse. The app only shows a small, dismissible reminder that a key protects the login, but reusing the same password is still risky.
- Web, enterprise provisioning.

## Later

- Packaging and signing per OS will be decided once the first version is finished.
