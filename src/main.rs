#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_alloc::Heap;

use pfp as _; // global logger + panicking-behavior + memory layout
use pfp::microbit::*;

static mut SEND_BUF: [u8; 1] = *b"1";
static mut RECV_BUF: [u8; 1] = [0; 1];

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[entry]
fn main() -> ! {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 128; // Heap Size in bytes
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    unsafe {
        let uBit = ffi::create_microbit();
    };

    defmt::panic!("End");
}
