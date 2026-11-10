# 1.0 platforms and test hardware

**Date:** 2026-11-09
**Status:** accepted

## Decision

- **1.0 ships on Linux and Windows** (x86-64 and ARM64). Development order: Linux, then Windows.
- **macOS is not part of 1.0.** It happens only if an external contributor with Mac hardware takes it on ([M7](../roadmap/phase-7-macos.md)). This also leaves iOS, which builds through Xcode on macOS, without a path until then.
- **Only the YubiKey 5 (two, bought for the project on the latest firmware) and the Security Key are tested on hardware.** The YubiKey 4 and YubiKey 5 firmware before 5.7 are known gaps.
- **Linux: Wayland sessions only.** X11 is not supported; Slint is built without its X11 backend.
- **Only the latest OS release is tested:** the latest release of each Linux distro in the VM lab plus the current Ubuntu LTS, and the latest Windows 11. Older releases (including Windows 10) are not tested or supported.

## Why

The project can only build and test on the hardware it has: Linux and Windows machines, two YubiKey 5 and a Security Key. macOS and the YubiKey 4 would need an external contributor to ever be supported. Dropping X11 and older OS releases keeps the test matrix to what one person can run on VMs.

## Consequences

- `IDEA.md` → Scope and Platform & tech list Linux and Windows for 1.0; macOS moves to the roadmap.
- W10 ("Sign in to my computer") no longer leads with macOS account pairing; it ships with a Linux guide (D10) and a Windows answer (D11).
- Untested models are still handled by capability detection: the app offers only what each key reports it supports. Those paths are not run on hardware and are listed as untested in the 1.0 release notes.
- Pair flows that put codes, OpenPGP, PIV or challenge-response on two full keys run on the two YubiKey 5 available for testing and are release gates, not hardware gaps. Tests against the fake `KeyPort` don't replace them.
- No X11 handling is planned, not even a "needs Wayland" message: Linux Mint, the main distro still on X11 by default, gets Wayland support in its next release.
- The Linux build baseline (D12) is the oldest glibc among the tested releases, so it moves forward as distros release.
