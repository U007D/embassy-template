#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    {% if chip contains "nrf" -%}
    let peripherals = embassy_nrf::init(Default::default());
    {% endif -%}

    {% if chip contains "stm32" -%}
    let peripherals = embassy_stm32::init(Default::default());
    {% endif -%}

    {% if chip contains "rp2040" -%}
    let peripherals = embassy_rp::init(Default::default());
    {% endif -%}

    let mut led = Output::new(peripherals.PIN_0, Level::Low);
    loop {
        defmt::info!("Blink");

        led.set_high();
        Timer::after_millis(500).await;

        led.set_low();
        Timer::after_millis(500).await;
    }
}
