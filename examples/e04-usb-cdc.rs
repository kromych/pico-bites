//! Shows off the minimal required structure.
//!
//! This will print "Running" in the embed console when run with `cargo embed --example e00-alive`.
#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use defmt as log;
use rp2040_hal as hal;
use rp_pico as bsp;

use core::fmt::Write;
use heapless::String;
use usb_device::{class_prelude::*, prelude::*};
use usbd_serial::SerialPort;
use usbd_serial::USB_CLASS_CDC;

const USB_VENDOR_ID: u16 = 0x16c2;
const USB_PRODUCT_ID: u16 = 0x27df;
const USB_DEV_MANUFACTURER: &'static str = "Fake company";
const USB_DEV_PRODUCT: &'static str = "Serial port";
const USB_DEV_SERIAL_NUMBER: &'static str = "TEST";

#[bsp::entry]
fn main() -> ! {
    log::info!(
        "Board {}, git revision {:x}, ROM verion {:x}",
        hal::rom_data::copyright_string(),
        hal::rom_data::git_revision(),
        hal::rom_data::rom_version_number(),
    );

    // Grab our singleton objects
    let mut pac = bsp::hal::pac::Peripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    // Set up the USB driver
    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    // Set up the USB Communications Class Device driver
    let mut serial = SerialPort::new(&usb_bus);

    // Create a USB device with a fake VID and PID
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(USB_VENDOR_ID, USB_PRODUCT_ID))
        .manufacturer(USB_DEV_MANUFACTURER)
        .product(USB_DEV_PRODUCT)
        .serial_number(USB_DEV_SERIAL_NUMBER)
        .device_class(USB_CLASS_CDC)
        .build();

    let mut said_hello = false;
    loop {
        // Check for new data
        if usb_dev.poll(&mut [&mut serial]) {
            let mut buf = [0u8; 64];
            match serial.read(&mut buf) {
                Err(_e) => {
                    // Do nothing
                }
                Ok(0) => {
                    // Do nothing
                }
                Ok(count) => {
                    log::info!("Read {} bytes", count);

                    if !said_hello {
                        said_hello = true;
                        let _ = serial.write(b"Hello, type to convert to the upper case!\r\n");

                        let time = timer.get_counter().ticks();
                        let mut text: String<64> = String::new();
                        write!(&mut text, "Timer ticks: {:#x}\r\n", time).unwrap();

                        // This only works reliably because the number of bytes written to
                        // the serial port is smaller than the buffers available to the USB
                        // peripheral. In general, the return value should be handled, so that
                        // bytes not transferred yet don't get lost.
                        let _ = serial.write(text.as_bytes());
                    }

                    // Convert to upper case
                    buf.iter_mut().take(count).for_each(|b| {
                        b.make_ascii_uppercase();
                    });
                    // Send back to the host
                    let mut wr_ptr = &buf[..count];
                    while !wr_ptr.is_empty() {
                        match serial.write(wr_ptr) {
                            Ok(len) => wr_ptr = &wr_ptr[len..],
                            // On error, just drop unwritten data.
                            // One possible error is Err(WouldBlock), meaning the USB
                            // write buffer is full.
                            Err(_) => break,
                        };
                    }
                }
            }
        }
    }
}
