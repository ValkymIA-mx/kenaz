# Recovery sheet renderer (D5)

**Date:** 2026-11-09
**Status:** accepted for the Linux-first implementation; portability is checked in the later platform gates.

## Decision

Embed the Typst compiler and PDF exporter in the Rust SHEET implementation, with the template and licensed fonts bundled. Use a restricted in-memory `World` to supply the template and sheet data. Do not ship the Typst CLI as an application runtime requirement. Keep the CLI for template editing and spike comparison.

## Evidence and trade-off

The [M0 experiment](../reports/2026-11-09-m0-sheet.md) rendered the existing Spanish and English template with both the installed Typst CLI (0.15.1) and embedded crates (0.15.0), offline with system and embedded-default fonts excluded. All four PDFs retained the references' three US Letter pages and checked content. The generated QR was independently decoded from all four rasterized PDFs and matched the complete sample provisioning URI. The embedded English PDF embeds the four bundled font faces.

Both routes work. Embedding avoids locating/versioning a second executable and lets the renderer receive real secrets directly in memory, without a temporary plaintext JSON file or an external compiler process. The cost is a substantial Rust compiler dependency graph and responsibility for maintaining `World`, fonts, diagnostics and compiler updates. The CLI has a simpler process interface and isolates compiler failures in another process, but would need separate deployment and a safe secret-input channel. No user data is sent to a service with either route.

The choice is not based on a speed claim: the one recorded host run is not a controlled benchmark, and the CLI and library patch versions differ. No ARM64, guest, Windows, physical-printer or exhaustive layout result is implied.

## Consequences

M1 SHEET starts from the embedded route, retaining only the template/data/font file access it actually needs. The M0 standalone spike intentionally writes public example data to its ignored output directory for comparison; **it is not a production secret-handling design**. Production accepts real data in memory and writes only the requested PDF, never a plaintext intermediate. Compiler dependencies and bundled fonts must be included in licence review. Full-sheet visual, print, translation and platform acceptance remain later gates.
