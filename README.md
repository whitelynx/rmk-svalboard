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
