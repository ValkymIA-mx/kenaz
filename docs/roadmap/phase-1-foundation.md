# M1: Foundation

**Goal:** the production skeleton. Every lane gets its crate, the contracts in [overview](overview.md#contracts-frozen-in-m1) are frozen, and the app detects a real key on Linux.

**Depends on:** M0 decisions D1, D5, D7; U0 findings for DESIGN.

## PLATFORM

- [ ] Cargo workspace: `crates/key`, `crates/vault`, `crates/core`, `crates/sheet`, `crates/app`; `content/` for strings and site data.
- [ ] CI: build + test for Linux x86-64 and aarch64 in the same baseline images as local guest builds, `rustfmt`, `clippy`, licence check for GPLv3 compatibility (LGPL/MIT/Apache deps).
- [ ] D12 decided; `rust-toolchain.toml`, baseline Containerfiles and `Cross.toml` for x86-64 and aarch64 as described in the [development environment](../runbooks/dev-environment.md#build-baseline-for-guests); ARM64 guest LX5 added to the VM lab.
- [ ] App data folder on Linux (XDG).
- [ ] Keep OS-specific code behind small modules (data folder, screen capture, process checks) so M5 (and a contributor's macOS port) add implementations instead of reshaping callers.

## KEY

- [ ] `KeyPort` trait and the `yubikit` implementation: discovery, hotplug events, device info, recognising the same key when re-plugged (by serial).
- [ ] Error taxonomy: not a YubiKey, busy in another program (`scdaemon`), `pcscd` not running, no permission for the device, wrong PIN with tries left, applet missing on this model.
- [ ] In-memory fake `KeyPort` for CORE and UI tests.

## VAULT

- [ ] `SecretsStore`: versioned entry schema inside the age payload, load/save in the app data folder, one unlock identity working end to end. Saves are atomic (write a new file, flush it, then replace the old one), so a crash mid-save leaves the previous file intact; "committed" means the replace has finished ([decision](../decisions/2026-11-09-secret-save-order.md)).
- [ ] Memory discipline: secrets only in zeroizing types; no plaintext path to disk.
- [ ] PIN rule (6–8 ASCII) as a validator usable while typing; admin code generator (8 printable ASCII, no byte above `0x7F`).

## CORE

- [ ] Non-secret app state: key registry (label, role main/spare, serial, model), checklist entries, settings; stored in the app data folder.
- [ ] D8 decided (when PIV/OpenPGP PINs leave factory values).

## UI

- [ ] Slint on Linux built with the Wayland backend only (`backend-winit-wayland`, no X11 backend).
- [ ] App shell: navigation, design tokens from DESIGN as Slint globals, home screen frame ("What do you want to protect?"), advanced details toggle.
- [ ] Component library from the DESIGN inventory, each with accessible role and name and a keyboard focus order, shown on a gallery screen in es and en.
- [ ] Key status strip: shows the plugged key(s) by label and model, live on hotplug.
- [ ] String pipeline: extract `@tr` strings to `.po`, load es/en at runtime, language follows the OS with a manual override.

## DESIGN

- [ ] D13 (visual direction), D14 (accessibility target), D15 (step illustrations), D16 (W2 window layout), D17 (key identity) decided.
- [ ] Design tokens and component inventory frozen (see [contract](overview.md#contracts-frozen-in-m1)), including warning levels agreed with the CONTENT tone guide: information, caution, danger, can't be undone.
- [ ] Interaction patterns for the hardware moments: waiting for a key, swapping keys on one port, "touch your key", writing to a key (don't unplug), and what happens when a key is pulled mid-step.
- [ ] Wireframes for W1–W5 in es and en, every state included; reviewed with CORE (state machines match) and CONTENT (copy fits).
- [ ] App icon, and the printed recovery sheet's look aligned with the app (`docs/brand/`).

## CONTENT

- [ ] Copy style guide (plain language rules, banned jargon list, tone for warnings).
- [ ] Site list schema frozen; data validated in CI.

## SHEET

- [ ] `crates/sheet`: Rust types matching `example.json`, QR SVG generation, PDF bytes out; opened with the OS viewer for printing.

## Exit criteria

- The baseline build starts on every Linux guest in the lab so far (no missing libraries, glibc within the baseline), shows the plugged YubiKey's model in es and en, and reacts to unplug/replug; the aarch64 build does the same in LX5.
- Secrets file round trip (create, add entry, close, reopen with the card) passes on real hardware.
- `crates/sheet` renders `example.json` to a PDF matching the template output.
- The component gallery works keyboard-only and meets the D14 screen reader level with Orca on LX1, in es and en.
- W1–W5 wireframes reviewed and signed off by CORE, CONTENT and UI.
