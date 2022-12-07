#![no_main]
#![no_std]

#[cfg(all(feature = "v1", feature = "v2"))]
compile_error!("Only one version of the micro:bit can be selected");

use pfp::{
    self as _,
    radio::{
        MICROBIT_RADIO_BASE_ADDRESS, MICROBIT_RADIO_CRCINIT, MICROBIT_RADIO_CRCPOLY,
        MICROBIT_RADIO_DEFAULT_FREQUENCY, MICROBIT_RADIO_DEFAULT_TX_POWER,
        MICROBIT_RADIO_HEADER_SIZE, MICROBIT_RADIO_MAX_PACKET_SIZE,
    },
}; // global logger + panicking-behavior + memory layout

use core::sync::atomic::{compiler_fence, Ordering};
use cortex_m_rt::entry;
use microbit::{
    hal::{prelude::*, Timer},
    pac::RADIO,
    Peripherals,
};

use pfp::{radio, serial};

pub fn enable_radio(peripherals: &Peripherals) -> core::fmt::Result {
    // Clock setup
    unsafe {
        // https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/source/drivers/MicroBitRadio.cpp#L291
        peripherals.CLOCK.events_hfclkstarted.write(|w| w.bits(0));
        // https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/source/drivers/MicroBitRadio.cpp#L292
        peripherals.CLOCK.tasks_hfclkstart.write(|w| w.bits(1));
        // Wait for the clock to start
        while peripherals.CLOCK.events_hfclkstarted.read().bits() == 0 {}
    }

    // Disables all interrupts, Nordic's code writes to all bits
    peripherals
        .RADIO
        .intenset
        .write(|w| unsafe { w.bits(0xFFFF_FFFF) });

    // use proprietary radio protocol
    // Source: <https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/source/drivers/MicroBitRadio.cpp#L301>
    peripherals.RADIO.mode.write(|w| w.mode().nrf_1mbit());

    peripherals
        .RADIO
        .shorts
        .write(|w| w.ready_start().enabled().end_disable().enabled());

    unsafe {
        // set power level
        peripherals
            .RADIO
            .txpower
            .write(|w| w.bits(MICROBIT_RADIO_DEFAULT_TX_POWER));

        // packet size is statically known
        peripherals
            .RADIO
            .pcnf0
            .write(|w| w.lflen().bits(0).s1len().bits(0).s0len().bit(false));

        peripherals.RADIO.pcnf1.write(|w| {
            w.maxlen()
                .bits(1 + MICROBIT_RADIO_HEADER_SIZE)
                .statlen()
                .bits(1) // number of bytes on air, static size
                // 4-Byte Base Address + 1-Byte Address Prefix
                .balen()
                .bits(4)
                .whiteen()
                .set_bit()
                .endian()
                .big()
        });

        peripherals
            .RADIO
            .crcinit
            .write(|w| w.crcinit().bits(MICROBIT_RADIO_CRCINIT & 0x00FF_FFFF));

        peripherals
            .RADIO
            .crcpoly
            .write(|w| w.crcpoly().bits(MICROBIT_RADIO_CRCPOLY & 0x00FF_FFFF));

        // Source: <https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/source/drivers/MicroBitRadio.cpp#L330>
        peripherals.RADIO.crccnf.write(|w| w.len().two());

        peripherals.RADIO.base0.write(|w| w.bits(0xe7e7e7e7));
        peripherals.RADIO.base1.write(|w| w.bits(0x43434343));
        peripherals.RADIO.prefix0.write(|w| w.bits(0x23c343e7));
        peripherals.RADIO.prefix1.write(|w| w.bits(0x13e363a3));

        peripherals
            .RADIO
            .frequency
            .write(|w| w.frequency().bits(MICROBIT_RADIO_DEFAULT_FREQUENCY)); // 2407 MHz
    }

    Ok(())
}

pub fn send(radio: &RADIO, msg: &mut [u8; 1]) -> core::fmt::Result {
    // while radio.events_disabled.read().bits() != 0 {}

    // enable "disabled" interrupt
    radio.intenset.write(|w| w.disabled().set_bit());

    unsafe {
        radio.txaddress.write(|w| w.txaddress().bits(0));

        let mut p = core::slice::from_raw_parts(msg.as_mut_ptr(), 1);
        defmt::println!("sending {:#?}", p);
        radio.packetptr.write(|w| w.bits(p.as_ptr() as u32));
        radio.events_address.write(|w| w.bits(1));
        radio.events_disabled.reset();
        radio.events_ready.reset();
        radio.events_end.reset();
        radio.events_payload.write(|w| w.bits(0));

        compiler_fence(Ordering::Release);
        radio.tasks_txen.write(|w| w.bits(1));

        compiler_fence(Ordering::SeqCst);
        defmt::println!("Sending on RADIO...");
        while radio.events_disabled.read().bits() == 0 {}

        compiler_fence(Ordering::Acquire);
        defmt::info!("Sent");
    }
    Ok(())
}

pub fn send_complete(radio: &RADIO) {
    defmt::info!(
        "Disbaled? {} ready {} end {}",
        radio.events_disabled.read().bits(),
        radio.events_ready.read().bits(),
        radio.events_end.read().bits()
    );
    radio.intenclr.write(|w| w.disabled().set_bit());
    radio.events_disabled.reset();
    radio.events_ready.reset();
    radio.events_end.reset();
    compiler_fence(Ordering::SeqCst);
}

static mut SEND_BUF: [u8; 1] = *b"1";

#[entry]
fn main() -> ! {
    if let Some(peripherals) = Peripherals::take() {
        serial::enable_serial(&peripherals).unwrap();
        enable_radio(&peripherals).unwrap();
        let mut timer = Timer::new(peripherals.TIMER0);

        defmt::println!("Logger works!");
        loop {
            // serial::send(&peripherals.UART0, "Hello, world!\r\n").unwrap();
            // defmt::println!("Sent on UART...");
            // timer.delay_ms(500u32);
            // send(&peripherals.RADIO, b"Hello! micro:rust\r\n").unwrap();
            unsafe { send(&peripherals.RADIO, &mut SEND_BUF).unwrap() };
            send_complete(&peripherals.RADIO);
            defmt::println!("Sent on RADIO...");
            timer.delay_ms(2000u32);
        }
        defmt::panic!("End LOOP");
    }

    defmt::panic!("End");
}
