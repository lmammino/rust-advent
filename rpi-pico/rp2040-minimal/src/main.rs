#![no_main]
#![no_std]

use core::panic::PanicInfo;

// The reset handler
/// # Safety
/// this is low level unsafe pico code
#[no_mangle]
#[inline(always)]
pub unsafe extern "C" fn Reset() -> ! {
    let _x = 42;

    // can't return so we go into an infinite loop here
    loop {
        let res = y2021ex06::part1("3,4,3,1,2");
        assert_eq!(5934, res);
        cortex_m::asm::nop();
    }
}

// The reset vector, a pointer into the reset handler
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

// #[lang = "eh_personality"]
// extern "C" fn eh_personality() {}
