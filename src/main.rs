#![no_main]
#![no_std]

use cortex_m as _;
use defmt_rtt as _;
// use panic_halt as _;
use panic_abort as _;

use cortex_m_rt::entry;

mod microbit;
use microbit::*;

// static mut SEND_BUF: [u8; 1] = *b"1";
// static mut RECV_BUF: [u8; 1] = [0; 1];

#[entry]
fn main() -> ! {
    unsafe {
        defmt::println!("ping");
        let ping = ffi::ping();
        defmt::println!("pong {}", ping);
        let x = ffi::create_microbit();
        defmt::println!("ubit {}", x);
    }

    let mut time = 0;
    loop {
        // ~1s
        if time % 600000 == 0 {
            defmt::println!("loop");
            time = 0;
        }
        time += 1;
    }

    defmt::panic!("End");
}
