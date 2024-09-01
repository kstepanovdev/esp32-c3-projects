#![no_std]
#![no_main]

use core::fmt::Write;
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    gpio::{Io, Level, Output},
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
    uart::Uart,
};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);

    let clocks = ClockControl::max(system.clock_control).freeze();
    let delay = Delay::new(&clocks);
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    let mut uart0 = Uart::new(peripherals.UART1, &clocks, io.pins.gpio21, io.pins.gpio20).unwrap();
    let mut led = Output::new(io.pins.gpio7, Level::Low);

    esp_println::logger::init_logger_from_env();

    let mut blink_counter = 0;

    loop {
        led.toggle();
        blink_counter += 1;
        write!(uart0, "Blinked {:?} times\r\n", blink_counter).unwrap();
        delay.delay(500.millis());
    }
}
