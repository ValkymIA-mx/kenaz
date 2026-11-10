# Kenaz ᚲ

**ValkymIA Kenaz — your security key companion.**

[Español](README.es.md)

Kenaz is a free, open-source desktop app that walks everyday people through protecting their accounts with a physical security key (YubiKey): passkeys, 6-digit codes, and a spare key so losing one isn't a disaster. It explains, in plain language, what to do with the key, in what order, and how not to get locked out.

> **Status: design stage.** There is no app to install yet. This repository holds the product design, the roadmap and the recovery sheet template. Kenaz is a community tool with best-effort support.

## What it helps you do

### First: protect your accounts

- **Sign in with your key.** Add the key to your important accounts as a passkey (no password) or as a second step after your password. Email first, then banking, then social media.
- **6-digit codes.** Scan the QR code a site shows you and get your codes from the key, one click to copy.
- **A spare key from day one.** Everything goes on two keys, so losing one doesn't lock you out. Kenaz keeps a checklist of which sites still need the spare.
- **"Am I protected?"** One screen shows which sites are on both keys, what's missing, and how many PIN tries are left.
- **When things go wrong.** Guided steps to replace a lost key, unblock a PIN, or reset a key you're giving away, with plain warnings before anything is erased.

### Then, once your accounts are safe

- Unlock a password manager with the key (KeePassXC first).
- Sign in to your computer, use SSH, and sign PDFs.
- Encrypt and sign email (Thunderbird first).
- Stop the key from typing random letters when touched.
- Sign PDFs with Mexico's e.firma (SAT FIEL) stored on the key.

## How it keeps you from getting locked out

- **Two keys, set up together.** Secrets can't be copied off a YubiKey later, so Kenaz writes each one to both keys at once.
- **An encrypted secrets file.** Setup secrets are kept in one [age](https://age-encryption.org/v1) file that only your keys can open, so a replacement key can be set up from it. It can also be opened outside Kenaz with standard age tools.
- **A printable recovery sheet**, one per key, for when both the keys and the file are gone. It brings back your 6-digit codes and password manager unlock, but not your email encryption or certificate keys: those aren't printed, so they're lost with the keys and the file (an e.firma can be imported again from SAT's original files). PINs and admin codes are never printed; you write them by hand.
- **One PIN to remember.** Admin codes are random, stored in the secrets file, and shown once for the recovery sheet.

Passkeys can't be backed up this way: each website registers each key separately. Kenaz guides you through adding the spare on every site and tracks which ones are missing it.

## Who it's for

- People who reuse passwords and have never set up two-step login.
- People who were handed a YubiKey and don't know what to do with it.
- People who already protect their accounts and want the next step.

Not for security engineers, organisations rolling out keys to staff, or custom OpenPGP setups: [Yubico Authenticator](https://www.yubico.com/products/yubico-authenticator/), Kleopatra and `ykman` already cover them. Kenaz is a guided companion, not a replacement for them; the same keys keep working with the official tools.

## Platforms and keys

| | 1.0 | Later |
|---|---|---|
| Linux (Wayland), x86-64 and ARM64 | ✓ | |
| Windows, x86-64 and ARM64 | ✓ | |
| macOS | | Needs an external contributor |
| Android, iOS | | Roadmap |

- **Tested keys:** YubiKey 5 and Security Key.
- **Untested:** YubiKey 4 and YubiKey 5 with firmware before 5.7. Kenaz reads what each key supports and only offers that, but these paths aren't run on hardware.
- **Languages:** Spanish and English.

## The name

**Kenaz (ᚲ)** is the rune of the torch: knowledge and illumination. Kenaz sheds light, in plain words, on a topic that stays opaque to anyone who isn't technical: what to do with the key, in what order, and how not to get locked out.

**Pronunciation:** KEN-az (en) · KE-nas (es).

## Project documents

- [`IDEA.md`](IDEA.md): product design: scope, principles, workflows, backup rules, technology.
- [`docs/roadmap/overview.md`](docs/roadmap/overview.md): milestones, work lanes, open decisions, test plan.
- [`docs/decisions/`](docs/decisions/): recorded decisions.
- [`docs/runbooks/dev-environment.md`](docs/runbooks/dev-environment.md): build tools and the VM test lab.
- [`templates/recovery-sheet/`](templates/recovery-sheet/): the printable recovery sheet (Typst).

Built with Rust, [Slint](https://slint.dev) and Yubico's [`yubikit`](https://docs.rs/yubikit) crate.

## Support

Kenaz is a community project. Help is best-effort, with no guaranteed response time and no warranty. If you're locked out of an account, the site's own account recovery is the fastest way back in.

## Licence

Open source. The licence will be GPL-3.0, the terms under which the project uses Slint; the `LICENSE` file is added with the first code.

Kenaz is not affiliated with or endorsed by Yubico. YubiKey and Yubico are trademarks of Yubico AB.
