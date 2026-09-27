# AirCard Transit Scan Fix

Based on Lumid-Off/AirCard-Windows commit
`a0d546e05a04752d0da40105dc3b3528ced81757`.

## Fix

The scanner rejected identifiers containing more than two hyphens or one
underscore, although both symbols are valid in URL-safe Base64. This patch
removes that symbol-count rejection while retaining the other validation checks.
It adds synthetic regression tests for repeated symbols and for extracting a
SHA-1 identifier with three hyphens from the existing syslog format.

The application continues using its existing syslog reader and artwork-transfer
implementation. Its title is **AirCard - Transit Scan Fix**. The direct
phone-service test is ignored by default so automated tests do not need hardware.
No real card identifiers or device logs are included in the source or fixtures.

## Build and test

On Windows with Rust and a compatible linker:

```powershell
cargo test --release --locked -- --skip test_usbmux_query --skip test_list_connected_devices
cargo build --release --locked
```

The output is `target/release/aircard.exe`. The **Build Transit Scan Fix** GitHub
Actions workflow runs the tests, builds on a Windows MSVC runner, and uploads the
EXE, these notes, the MIT license and a SHA256 checksum as an artifact. It runs
when pushed to `codex/transit-scan-fix` and does not publish a release.

Four targeted Rust validator/regular-expression tests passed in a standalone
harness, reproducing the original rejection and the corrected acceptance. The
workflow provides the full Windows build and test results. An interactive test
with a connected iPhone is still needed.

## Try the fix

1. Connect and unlock the iPhone, then open **AirCard - Transit Scan Fix**.
2. Select the iPhone and start **Scan**.
3. Open the affected card in Wallet and open its Card Details.
4. Check that the card appears, then stop scanning before applying artwork.

The upstream MIT license is included.
