#![no_std]
#![no_main]

use core::fmt::Write;
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, delay::Delay, gpio::Io, peripherals::Peripherals, prelude::*,
    rtc_cntl::Rtc, system::SystemControl, timer::timg::TimerGroup, uart::Uart,
};
use esp_println::{print, println};
use heapless::Vec;
use nb::block;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    // disable watchdogs
    let mut rtc = Rtc::new(peripherals.LPWR, None);
    rtc.rwdt.disable();
    rtc.swd.disable();

    let timg0 = TimerGroup::new_async(peripherals.TIMG0, &clocks);
    let mut wdt0 = timg0.wdt;
    wdt0.disable();

    let timg1 = TimerGroup::new_async(peripherals.TIMG1, &clocks);
    let mut wdt1 = timg1.wdt;
    wdt1.disable();

    let uart0 = Uart::new(peripherals.UART0, &clocks, io.pins.gpio21, io.pins.gpio20).unwrap();
    let (mut tx, mut rx) = uart0.split();

    let delay = Delay::new(&clocks);

    let mut read_buffer: Vec<u8, 32> = Vec::new();

    loop {
        if read_buffer.len() >= 32 {
            println!("Read buffer has been overflown");
        }

        let byte = rx.read_byte().unwrap();
        println!("{}", byte as char);
        if byte == b'\n' {
            println!("{}", byte);
            for byte in read_buffer.iter() {
                println!("{:?}", *byte as char);
                println!("\r\n");
            }
            read_buffer.clear();
        } else {
            println!("{}", byte);
            read_buffer.push(byte).unwrap();
        }
        delay.delay_millis(200);
    }
}
