#![no_std]
#![no_main]

use defmt::{debug, println};
use embassy_executor::Spawner;
use embassy_stm32::{
    gpio::{AnyPin, Level, Output, Pin, Speed},
    time::Hertz,
    Config,
};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;

        config.rcc.hse = Some(Hse {
            freq: Hertz(25_000_000),
            mode: HseMode::Oscillator,
        });
        config.rcc.pll_src = PllSource::HSE;
        config.rcc.pll = Some(Pll {
            prediv: PllPreDiv::DIV25,  // 1Hz
            mul: PllMul::MUL336,       // 336Hz
            divp: Some(PllPDiv::DIV4), // 84Hz
            divq: Some(PllQDiv::DIV6), //48Hz
            divr: None,
        });
        config.rcc.ahb_pre = AHBPrescaler::DIV1;
        config.rcc.apb1_pre = APBPrescaler::DIV2;
        config.rcc.apb2_pre = APBPrescaler::DIV1;
        config.rcc.sys = Sysclk::PLL1_P;
        config.rcc.mux.clk48sel = mux::Clk48sel::PLL1_Q;
    };

    let p = embassy_stm32::init(config);
    debug!("test");

    _spawner.spawn(blink(p.PC13.degrade())).unwrap();
}

#[embassy_executor::task]
async fn blink(pin: AnyPin) {
    let mut led = Output::new(pin, Level::Low, Speed::High);

    loop {
        debug!("hello world");
        led.set_level(Level::High);
        Timer::after_millis(150).await;

        led.set_level(Level::Low);
        Timer::after_millis(150).await;
    }
}
