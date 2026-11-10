# M0: Spikes

**Goal:** answer the open technical questions in `IDEA.md` that matter on Linux, with throwaway code on real keys, before committing to a structure. Spike code is deleted or rewritten in M1; the output is the decisions. Windows questions (D6, elevation) wait for [M5](phase-5-windows.md). Hardware: YubiKey 5 and Security Key only (see [overview](overview.md#hardware-test-matrix)).

**Depends on:** nothing. All lanes start at once.

## KEY

- [ ] Pin `yubikit = "=1.0.0-beta.0"`. Enumerate keys over PC/SC (`pcscd`) and HID on Linux; read model, serial, firmware, form factor, NFC and enabled applets.
- [ ] Linux access without root: what happens when `pcscd` isn't running, and whether the distro's udev rules let a normal user open the FIDO HID device. Record the error each case gives.
- [ ] One real operation per applet: OATH list + calculate; CTAP2 `getInfo` + list stored passkeys; PIV import key + certificate into 9c and verify a test signature; OpenPGP key import; YubiOTP read slot config and swap slots.
- [ ] e.firma through `yubikit`: import the RSA 2048 key + `.cer` into 9c and verify a test signature on the YubiKey 5. (YubiKey 4 stays unchecked: no hardware.)
- [ ] `scdaemon`: reproduce "GnuPG holds the key" on Linux; find a reliable detection signal and the least invasive fix.
- [ ] D7: write down the `yubikit` gaps found and the fallback trigger.

## VAULT

- [ ] D1: implement a custom age `Identity` whose key step runs on the card. Prove compatibility both ways: a file written by the app opens with `age-plugin-yubikey` (PIV) and `age-plugin-fido2-hmac` (FIDO2), and files from those plugins open in the app. If the API doesn't allow it, prototype the plugin-protocol fallback.
- [ ] D2: compare the PIV, FIDO2 and OpenPGP options on the hardware matrix (which keys support each, plugin availability). Pick what ships in M2 and what the app recommends; note that the Windows admin prompt is unknown until M5 (see overview risk).
- [ ] D3: build a card-ready Curve25519 OpenPGP certificate in memory with each candidate library, import it to a key, and check GnuPG and Thunderbird accept it.
- [ ] D4: build a self-signed P-256 certificate for 9a with each candidate library; check SSH accepts it.

## UI + PLATFORM

- [ ] VM lab per the [development environment](../runbooks/dev-environment.md): hypervisor, guests LX1 and LX2, USB redirection of both key models, `clean` snapshots.
- [ ] Slint window with one `@tr` string switching es/en through gettext, run on Linux x86-64 and cross-built for aarch64.
- [ ] QR from screen on Linux through the Wayland screenshot portal (X11 is not supported), on LX1 and LX2, with the spike built in a container no newer than those guests (not natively on Arch). Record the permission prompts the user will see.
- [ ] D14 input: on LX1, check what Slint exposes to Orca (a button, a text field, a live "touch your key" message), keyboard-only navigation with visible focus, and fractional scaling (125%, 150%) under Wayland. Record what works and what needs upstream fixes.

## DESIGN

Not blocked by any spike; starts now so M1 wireframes rest on tested ideas.

- [ ] Information architecture: home screen, task list (W1–W5 first, W8–W13 appearing after the first protected account), codes, status, checklist, advanced details. One map of every screen and how the user gets to it.
- [ ] Journey map for W1, W2 and W6 from `IDEA.md`, marking every moment the user handles a key (plug, unplug, swap, touch) or leaves the app (browser, printer).
- [ ] Paper or clickable prototype of the home screen, W1 and W2; run U0 (see [overview](overview.md#usability-rounds)) and record the findings in `docs/reports/`.

## SHEET

- [ ] D5: render `recovery-sheet.typ` with `example.json` from Rust, offline, fonts bundled. Compare against `example-es.pdf` / `example-en.pdf`.
- [ ] Generate the `qr_svg` values from an `otpauth://` URI in Rust.

## CONTENT

Not blocked by any spike; starts now and runs through M2.

- [ ] Draft the site list schema and the first 10 sites per language (email providers first), with methods each site supports.
- [ ] Plain-language glossary: user term ↔ real term (for the advanced details view).

## Exit criteria

- D1–D5 and D7 recorded in `docs/decisions/`.
- `yubikit` go/no-go made; if no-go, M1 KEY plan rewritten for the fallback.
- U0 findings recorded; the screen map and home screen wording that M1 wireframes build on are agreed.
