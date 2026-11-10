# Development environment

Software needed to build Kenaz and test it on virtual machines. Development happens on an Arch Linux host; every test scenario (distros, desktop environments, ARM64 Linux, later Windows) runs in a VM with the YubiKeys passed through over USB.

Reference host: AMD Ryzen 9 7900X3D (24 threads), 90 GiB RAM, KVM available. Package names below are Arch's, checked against the repos on 2026-11-09.

## 1. Host build tools

| Need | Arch packages | Notes |
|---|---|---|
| Rust | `rustup` | Toolchain pinned in `rust-toolchain.toml` when the workspace is created (M1). Floor is **1.92**: `slint` 1.18 and `typst` 0.15 need it (`yubikit` 1.0.0-beta.0 needs 1.88). Components `rustfmt`, `clippy`; targets `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`. |
| C toolchain | `base-devel` `pkgconf` | |
| Slint (winit backend) | `fontconfig` `freetype2` `wayland` | Wayland only: Slint is built with `backend-winit-wayland` and no X11 backend, so no xcb or xkbcommon dev packages (per Slint's build guide). winit loads `libwayland-client` and `libxkbcommon` at runtime; every Wayland desktop ships them. |
| `yubikit` | `pcsclite` `systemd-libs` | `pcsc` crate links `libpcsclite`; on Linux `yubikit` uses `hidapi` (`linux-native`) and the `udev` crate, both on `libudev`. |
| Smart card service (spikes on the host) | `pcsclite` `ccid` | `systemctl enable --now pcscd.socket` |
| Translations | `gettext` + `cargo install slint-tr-extractor` | Extract `@tr` strings to `.pot`, merge into the es/en `.po` files. |
| Recovery sheet | `typst` `noto-fonts` `ttf-dejavu` | The template asks for Noto Sans / DejaVu Sans and DejaVu Sans Mono. The CLI is for editing the template; the app renders per D5. |
| Licence check | `cargo-deny` | |
| Guest test builds (x86-64 and aarch64) | `cross` `podman` | Every binary that runs in a guest is built in a baseline container, never natively on Arch (see below). |

### Build baseline for guests

A native Arch build links against Arch's glibc and system libraries, which are newer than Ubuntu LTS or Debian stable ship, so it can fail to start there with `GLIBC_2.xx not found`. Native host builds are for the host only (unit tests, spikes).

- **Baseline:** the tested release with the oldest glibc (D12 in the [roadmap](../roadmap/overview.md#decisions-to-record)); with the version policy below that is usually Ubuntu LTS or Debian stable. Run `ldd --version` in each Linux guest, record the result here, and re-check whenever a guest moves to a new release.
- **Images:** one Containerfile per target in the repository, `FROM` that distro release (pinned, not `latest`), with the dev packages installed: `libpcsclite-dev`, `libudev-dev`, `libfontconfig-dev`, `libwayland-dev` (or the distro's names). For aarch64 the image uses multiarch (`dpkg --add-architecture arm64`) and the cross linker.
- **Build:** `cross build --target x86_64-unknown-linux-gnu` and `--target aarch64-unknown-linux-gnu`, with `Cross.toml` pointing each target at its Containerfile.
- **Check before a guest run**, on the host:

  ```sh
  objdump -T target/<triple>/debug/kenaz | grep -o 'GLIBC_[0-9.]*' | sort -Vu | tail -1   # must be ≤ the baseline's glibc
  ```

  and in each guest:

  ```sh
  ldd ./kenaz | grep 'not found'   # must print nothing
  ```

  Slint's winit backend loads the Wayland libraries at runtime, which `ldd` doesn't show, so the last check is launching the app in every guest's session.

The same baseline is the starting point for packaging (D9).

## 2. Host reference and compatibility tools

Used to inspect and reset test keys, and to prove that files and keys the app writes work in other tools.

| Tool | Arch package | Used for |
|---|---|---|
| `ykman` | `yubikey-manager` | Inspect keys, reset applets between scenarios, cross-check what the app wrote |
| `age`, `rage` | `age` `rage-encryption` | Open the secrets file outside the app |
| age plugins | `age-plugin-yubikey`; AUR `age-plugin-fido2-hmac`, `age-plugin-openpgp-card` | D1 compatibility, "open outside the app" checks |
| GnuPG | `gnupg` | OpenPGP checks; reproducing `scdaemon` holding the key |
| PKCS#11 | `opensc` `yubico-piv-tool` (ykcs11) | PIV certificates in SSH and PDF signers |
| FIDO2 | `libfido2` | `fido2-token` to list credentials independently of `yubikit` |
| Test codes | `oath-toolkit` `qrencode` | Make `otpauth://` URIs and QR codes; check codes with `oathtool --totp` |

## 3. VM lab

### Hypervisor

```sh
sudo pacman -S qemu-full libvirt virt-manager virt-install edk2-ovmf edk2-aarch64 swtpm dnsmasq spice-gtk virtiofsd
sudo usermod -aG libvirt "$USER"
sudo systemctl enable --now libvirtd.socket   # or virtqemud.socket on a modular libvirt setup
```

Windows guests also need the `virtio-win` driver ISO (AUR `virtio-win`).

- x86-64 guests run under KVM.
- ARM64 guests run under QEMU emulation (TCG) with `edk2-aarch64` UEFI firmware: correct but several times slower. Give them a light Wayland compositor.
- Every guest gets a `qemu-xhci` USB controller (the `virt` machine type on ARM64 has none by default) and a SPICE display for USB redirection.

### Supported versions

- **Wayland sessions only.** X11 is not supported or tested.
- **Only the latest release** of each distro and of Windows 11 is tested, plus the current **Ubuntu LTS**. When a new release ships, the guest moves to it: reinstall, retake the `clean` snapshot, re-check the build baseline.
- While the latest Ubuntu release is the LTS (from each April LTS release to the next interim release), LX1 covers both and LX4 is skipped.

### Guest matrix

| VM | Arch | Runs on | Desktop / session | Why | Needed from |
|---|---|---|---|---|---|
| LX1 | x86-64 | KVM | Ubuntu LTS, GNOME, Wayland | Largest share of the audience; GNOME screenshot portal; likely build baseline | M0 |
| LX2 | x86-64 | KVM | Fedora Workstation (latest), GNOME, Wayland | Newest stack; `pcscd` polkit rules; newest systemd udev rules | M0 |
| LX3 | x86-64 | KVM | Fedora KDE Plasma (latest), Wayland | Okular and Kleopatra at home; KDE screenshot portal | M2 (screen QR), M4 |
| LX4 | x86-64 | KVM | Ubuntu (latest interim release), GNOME, Wayland | Newest Ubuntu packages ahead of the next LTS | M2 |
| LX5 | ARM64 | TCG | Debian stable (latest), labwc, Wayland | ARM64 Linux target; labwc is the compositor Raspberry Pi OS uses, and it stays light under emulation | M1 |
| WIN1 | x86-64 | KVM | Windows 11, latest release (UEFI Secure Boot + TPM via `swtpm`) | Windows target | M5 |
| WIN2 | ARM64 | TCG | Windows 11 ARM64, latest release | Windows ARM64 target; may be too slow for full passes (see open items) | M5 |

Sizing to start with: Linux x86-64 guests 4 vCPU / 8 GiB / 60 GiB disk; Windows 6 vCPU / 12 GiB / 80 GiB; ARM64 guests 4 vCPU (multi-threaded TCG) / 6 GiB / 60 GiB. Disks are qcow2 so snapshots work.

### Software in Linux guests

Install from the distro where packaged, from upstream releases otherwise:

- `pcsc-lite` + `ccid` with `pcscd` running.
- Firefox and Chromium (or Chrome), for passkey registration.
- Thunderbird, KeePassXC, GnuPG, OpenSSH, OpenSC/ykcs11, Okular. Kleopatra on LX3.
- `ykman`, `age`/`rage` and the three age plugins.
- The screenshot portal backend for the desktop: `xdg-desktop-portal-gnome` on LX1, LX2 and LX4, `xdg-desktop-portal-kde` on LX3, `xdg-desktop-portal-wlr` on LX5.
- A virtiofs share of the host's `target/` directory (scp works too). Only baseline builds from `target/x86_64-unknown-linux-gnu/` or `target/aarch64-unknown-linux-gnu/` run in guests; a plain `cargo build` on Arch lands in `target/debug/` and is for the host only.

### Passing YubiKeys through

A key can be in one place at a time: the host or one VM.

1. Free the key on the host first, or the VM can't claim it:

   ```sh
   sudo systemctl stop pcscd.socket pcscd.service
   gpgconf --kill scdaemon
   ```

2. **Interactive:** in virt-manager, *Virtual Machine → Redirect USB device*, pick the key.
3. **Physical replug that lands back in the VM:** open the guest with `remote-viewer` and an auto-redirect filter for Yubico (vendor `0x1050`), so a key plugged in physically goes straight to the guest:

   ```sh
   remote-viewer --spice-usbredir-auto-redirect-filter='-1,0x1050,-1,-1,1|-1,-1,-1,-1,0' spice://127.0.0.1:5900
   ```

   This covers the scenarios that need a real unplug: W1's "unplug this key and plug in the spare", and the passkey reset, which the key only accepts within seconds of being plugged in. Detaching the device from the VM probably doesn't power-cycle the key (not yet verified), so for resets use a physical replug.
4. **Scripted:** `virsh attach-device <vm> key.xml --live` / `virsh detach-device` with a USB `hostdev`. The two YubiKey 5 have the same vendor and product ID, so pick them by bus and device number from `lsusb -d 1050:`. Those numbers change on every replug.
5. The product ID changes when the key's enabled interfaces change (`ykman config usb`); redirect the key again afterwards.

### Snapshots and key state

- Take a `clean` snapshot of each guest after the OS and the tools above are installed; revert to it before each scenario.
- Keys can't be snapshotted. Reset what a scenario touched with `ykman`, run on the host or in the guest that holds the key:

  | Applet | Command |
  |---|---|
  | Passkeys (FIDO2) | `ykman fido reset` (within seconds of plugging in; touch required) |
  | 6-digit codes | `ykman oath reset` |
  | PIV | `ykman piv reset` |
  | OpenPGP | `ykman openpgp reset` |
  | OTP slots | `ykman otp swap` restores the factory layout after a W8 "move to long press" test |

- **W8 "remove it" can't be undone.** Deleting the factory Yubico OTP credential is permanent: `ykman otp yubiotp` can program a new credential, but not the factory-registered one. Test removal on one key only, as late as possible.

### Test accounts

- Passkeys without real accounts: [webauthn.io](https://webauthn.io) and [demo.yubico.com](https://demo.yubico.com/webauthn-technical/registration).
- 6-digit codes: generate `otpauth://` URIs and QR codes locally (`qrencode`), and compare the app's codes with `oathtool --totp`.
- Curated-site flows (W2, W4, W6): dedicated test accounts per site, email provider first. Never use personal accounts or personal keys. Site backup codes and test passwords stay outside the repository.

## 4. Windows guests (from M5)

Inside WIN1:

- `rustup` with the MSVC toolchain; Visual Studio Build Tools, "Desktop development with C++" (MSVC + Windows SDK), plus the ARM64 build tools for `aarch64-pc-windows-msvc`. Build both targets on WIN1 and run the ARM64 binary on WIN2.
- The Windows Smart Card service is built in (nothing like `pcscd` to install).
- Through `winget` or upstream installers: Gpg4win (GnuPG + Kleopatra), Thunderbird, KeePassXC, Okular, `ykman`, `age`/`rage` and `age-plugin-yubikey`. The OpenSSH client is built in.
- Two local accounts: a standard user (the admin prompt asks for an administrator password) and an administrator (a Yes/No prompt, what most home users see). See open items for which one is the default.

## Open items

- **Admin prompt variants:** home users are usually administrators and get a Yes/No prompt; a standard user gets a credentials prompt. Decide in M5 which account WIN1 tests by default.
- **Windows 11 ARM64 under emulation** may be too slow for full W1–W13 passes. Fallback: real ARM64 hardware, or the M6 waiver for Windows ARM64 full passes.
- **ARM64 Linux desktop:** labwc is chosen for speed under emulation; GNOME and KDE on ARM64 stay untested unless performance allows another guest.
