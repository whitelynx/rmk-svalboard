# Svalboard RMK Firmware

[![Build RMK firmware](https://github.com/whitelynx/rmk-svalboard/actions/workflows/build.yml/badge.svg)](https://github.com/whitelynx/rmk-svalboard/actions/workflows/build.yml)

This is the [RMK][] version of the firmware for the [Svalboard][].


## Status

**Note: This firmware is NOT ready for everyday use yet! It is not well tested, and is still missing several features:**

- Pointing device support is still unimplemented
- Status LEDs are not supported yet


## Using the Firmware

This firmware is automatically built using [GitHub Actions][]. To get the latest firmware, select the most recent run on that page (the first one in the list), scroll to the bottom of the page, and download the `firmware_uf2` artifact, then extract the `.uf2` file from the downloaded `.zip`.

To flash, press the `Reset` button on the bottom of the Svalboard twice, then copy the `.uf2` file to the USB drive that appears. See [the Svalboard docs][] for more information.

[RMK]: https://github.com/HaoboGu/rmk
[Svalboard]: https://svalboard.com/
[GitHub Actions]: https://github.com/whitelynx/rmk-svalboard/actions/workflows/build.yml
[the Svalboard docs]: https://docs.google.com/document/d/1Um4EAIK-GLQGw-9xHUFe-aCtHJDENYUSzhcqQi9ppwU/edit?tab=t.0#heading=h.ydo2kn17mbtt

### Building locally

1. Install [probe-rs](https://github.com/probe-rs/probe-rs)

   ```shell
   # Linux/macOS
   curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh

   # Windows
   irm https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.ps1 | iex
   ```

2. Build the firmware

   ```shell
   cargo build --release
   ```

3. Flash using debug probe

   If you have a debug probe connected to your rp2040 board, flashing is quite simple: run the following command to automatically compile and flash RMK firmware to the board:

   ```shell
   cargo run --release --bin central
   ```

4. (Optional) Flash using USB

   If you don't have a debug probe, you can flash the UF2 file to each side using the RP2040's bootloader:

   1. Connect the target side of the keyboard directly to the computer via USB.
   2. Enter the bootloader, by holding the `BOOT` button, pressing and releasing `RESET`, and then releasing `BOOT`.
   3. Open the USB drive that appears. (mounting the drive first if necessary)
   4. Copy either `rmk-svalboard-central.uf2` or `rmk-svalboard-peripheral.uf2` to the USB drive, according to which part of the keyboard is currently connected.

