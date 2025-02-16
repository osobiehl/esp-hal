//! SPI loopback test
//!
//! Folowing pins are used:
//! SCLK    GPIO12
//! MISO    GPIO11
//! MOSI    GPIO13
//! CS      GPIO10
//!
//! Depending on your target and the board you are using you have to change the
//! pins.
//!
//! This example transfers data via SPI.
//! Connect MISO and MOSI pins to see the outgoing data is read as incoming
//! data.

#![no_std]
#![no_main]

use core::fmt::Write;

use esp32s3_hal::{gpio::IO, pac::Peripherals, prelude::*, Delay, RtcCntl, Serial, Timer};
use panic_halt as _;
use xtensa_lx_rt::entry;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();

    // Disable the watchdog timers. For the ESP32-C3, this includes the Super WDT,
    // the RTC WDT, and the TIMG WDTs.
    let mut rtc_cntl = RtcCntl::new(peripherals.RTC_CNTL);
    let mut timer0 = Timer::new(peripherals.TIMG0);
    let mut serial0 = Serial::new(peripherals.UART0).unwrap();

    timer0.disable();
    rtc_cntl.set_wdt_global_enable(false);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let sclk = io.pins.gpio12;
    let miso = io.pins.gpio11;
    let mosi = io.pins.gpio13;
    let cs = io.pins.gpio10;

    let mut spi = esp32s3_hal::spi::Spi::new(
        peripherals.SPI2,
        sclk,
        mosi,
        miso,
        cs,
        100_000,
        embedded_hal::spi::MODE_0,
        &mut peripherals.SYSTEM,
    );

    let delay = Delay::new();

    loop {
        let mut data = [0xde, 0xca, 0xfb, 0xad];
        spi.transfer(&mut data).unwrap();
        writeln!(serial0, "{:x?}", data).ok();

        delay.delay(250_000);
    }
}
