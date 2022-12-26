#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_rp::adc::{Adc, Config};
use embassy_rp::interrupt;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::task]
async fn logger_task() {
    log::info!("test");
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let irq = interrupt::take!(ADC_IRQ_FIFO);
    let mut adc = Adc::new(p.ADC, irq, Config::default());

    let mut p26 = p.PIN_26;
    let mut p27 = p.PIN_27;
    let mut p28 = p.PIN_28;

    spawner.spawn(logger_task()).unwrap();
    let mut counter = 0;
    loop {
        counter += 1;
        log::info!("Tick {}", counter);
        Timer::after(Duration::from_secs(1)).await;
    }
}
