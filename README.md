# Various small examples to start learning embedded development with Pi Pico

## How to build and run

## Serial via Tigard

```sh
picocom -b 115200 -f n -d 8 -s 1 --imap lfcr /dev/tty.usbserial-TG11060e0 
```

## Projects I have learned from

* [RTIC and Serial](https://github.com/joaocarvalhoopen/Raspberry_Pi_Pico_in_Rust__Proj_Template_with_RTIC_USB-Serial_UF2)
* [dactyl-manuform-kb2040-rs](https://github.com/Nashenas88/dactyl-manuform-kb2040-rs/blob/main/src/main.rs#L80)
* [Pico probe rs](https://github.com/korken89/pico-probe/tree/master/src)
* [rp2040-usb-sound-card](https://github.com/mgottschlag/rp2040-usb-sound-card/blob/b8078b57361c1b08755e5ab5f9992c56457ec18b/src/main.rs#L188)
