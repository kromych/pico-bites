# Various small examples to start learning embedded development with Pi Pico

## How to build and run

One-time tool install to deploy:

```sh
cargo install elf2uf2-rs
```

To deploy an example:

```sh
cargo build --release --example e06-ws2812b
# -v for the verbosity, add -s for outputting serial data after deploying
# the firmware.
elf2uf2-rs -v target/thumbv6m-none-eabi/release/examples/e06-ws2812b -d
```

If the runner is set as `runner = "elf2uf2-rs -d"` in [.cargo/config.toml],
then `cargo run` will do that automatically.

That produces the UF2 binary that can be copied onto the board
after powering it on with the BOOT button held down. If your board
has the RUN button, can alternatively hold the BOOT button down and
press RUN instead of power cycling.

To debug with [Pico probe or Debug probe](https://github.com/raspberrypi/picoprobe)
and upload the firmware through it, here is a plethora of tools capable of that, and
either of the list can suffice.

The runner is set to `probe-run` by default so the Pico has to be connected though
a debug probe.

```sh
cargo install cargo-embed
cargo install cargo-flash
cargo install flip-link
cargo install probe-rs-cli
cargo install probe-rs-debugger
cargo install probe-run
```

## Serial via Tigard

```sh
picocom -b 115200 -f n -d 8 -s 1 --imap lfcr /dev/tty.usbserial-TG11060e0 
```

## Projects I have learned from

* [RTIC and Serial](https://github.com/joaocarvalhoopen/Raspberry_Pi_Pico_in_Rust__Proj_Template_with_RTIC_USB-Serial_UF2)
* [dactyl-manuform-kb2040-rs](https://github.com/Nashenas88/dactyl-manuform-kb2040-rs/blob/main/src/main.rs#L80)
* [Pico probe rs](https://github.com/korken89/pico-probe/tree/master/src)
* [rp2040-usb-sound-card](https://github.com/mgottschlag/rp2040-usb-sound-card/blob/b8078b57361c1b08755e5ab5f9992c56457ec18b/src/main.rs#L188)
