//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use embedded_time::fixed_point::FixedPoint;
use panic_probe as _;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

use core::fmt::Write;

#[entry]
fn main() -> ! {
    let mut allotofdata: [u8; 128 * 1024] = [0_u8; 128 * 1024];

    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let mut core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    // let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        //external_xtal_freq_hz,
        bsp::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led_pin = pins.led.into_push_pull_output();
    let mut gp15 = pins.gpio15.into_push_pull_output();

    let _uart_pins = (
        // UART TX (characters sent from RP2040) on pin 1 (GPIO0)
        pins.gpio0.into_mode::<bsp::hal::gpio::FunctionUart>(),
        // UART RX (characters received by RP2040) on pin 2 (GPIO1)
        pins.gpio1.into_mode::<bsp::hal::gpio::FunctionUart>(),
    );
    let mut uart = bsp::hal::uart::UartPeripheral::new(pac.UART0, &mut pac.RESETS)
        .enable(
            bsp::hal::uart::common_configs::_9600_8_N_1,
            clocks.peripheral_clock.freq(),
        )
        .unwrap();

    core.SYST.set_reload(0x00FF_FFFF);
    core.SYST.clear_current();
    core.SYST.enable_counter();

    // let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());

    uart.write_full_blocking(b"UART started\r\n");

    writeln!(uart, "freq  {}\r\n", clocks.system_clock.freq().integer()).unwrap();
    let mut loop_counter = 0;
    let input = include_str!("../input.txt");
    // let _str = "hello".to_string();
    loop {
        writeln!(uart, "loop  {}\r", loop_counter).unwrap();
        led_pin.set_high().unwrap();
        gp15.set_high().unwrap();
        // delay.delay_ms(100);

        let start = cortex_m::peripheral::SYST::get_current();
        let res = y2021ex06::part1(input);
        let end = cortex_m::peripheral::SYST::get_current();
        let delta;
        if start < end {
            delta = 0x00FF_FFFF + start - end
        } else {
            delta = start - end;
        }
        let start2 = cortex_m::peripheral::SYST::get_current();
        let res2 = y2021ex06::part2(input);
        let end2 = cortex_m::peripheral::SYST::get_current();
        let delta2;
        if start2 < end2 {
            delta2 = 0x00FF_FFFF + start2 - end2
        } else {
            delta2 = start2 - end2;
        }

        led_pin.set_low().unwrap();
        gp15.set_low().unwrap();
        // delay.delay_ms(100);

        allotofdata[(res % 1024) as usize] = (res % 256) as u8;

        writeln!(uart, "part1 {}         time {} us\r", res, delta).unwrap();
        writeln!(uart, "part2 {}  time {} us\r\n", res2, delta2).unwrap();
        loop_counter += 1;
    }
}

// End of file
