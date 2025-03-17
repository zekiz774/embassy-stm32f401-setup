#![no_std]
#![no_main]

use defmt::println;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{AnyPin, Level, Output, Pin, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::task]
async fn blink(pin: AnyPin) {
    let mut led = Output::new(pin, Level::Low, Speed::High);

    loop {
        println!("hello world");
        led.set_level(Level::High);
        Timer::after_millis(150).await;

        led.set_level(Level::Low);
        Timer::after_millis(150).await;
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let pin = embassy_stm32::init(Default::default());
    _spawner.spawn(blink(pin.PC13.degrade())).unwrap()
}
