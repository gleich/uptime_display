#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::*;
use embedded_hal::digital::v2::OutputPin;
use embedded_time::duration::*;
use rp_pico::hal;
use rp_pico::hal::pac;
use rp_pico::hal::prelude::*;
use {defmt_rtt as _, panic_probe as _};

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

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

    let sio = hal::Sio::new(pac.SIO);

    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());
    let mut led = pins.led.into_push_pull_output();

    info!("Setup everything");

    loop {
        led.set_high().expect("Failed to set LED high");
        delay.delay_ms(30);
        led.set_low().expect("Failed to set LED low");
    }
}
