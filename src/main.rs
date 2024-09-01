#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    gpio::{Io, Level, Output},
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);

    let clocks = ClockControl::max(system.clock_control).freeze();
    let delay = Delay::new(&clocks);

    let io = Io::new(peripherals.GPIO.into(), peripherals.IO_MUX.into());

    let mut led = Output::new(io.pins.gpio7, Level::Low);

    esp_println::logger::init_logger_from_env();

    loop {
        led.toggle();
        delay.delay_millis(500);
    }
}
