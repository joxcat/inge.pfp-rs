#![no_main]
#![no_std]

#[cfg(all(feature = "v1", feature = "v2"))]
compile_error!("Only one version of the micro:bit can be selected");

use pfp::{self as _, radio::*}; // global logger + panicking-behavior + memory layout

use cortex_m_rt::entry;
use microbit::{
    hal::{prelude::*, Timer},
    pac::RADIO,
    Peripherals,
};

use pfp::serial;

pub fn enable_radio(peripherals: &Peripherals) -> core::fmt::Result {
    // Clock setup
    unsafe {
        // Source: <https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/source/drivers/MicroBitRadio.cpp#L291>
        peripherals.CLOCK.events_hfclkstarted.write(|w| w.bits(0));
        // Source: <https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/source/drivers/MicroBitRadio.cpp#L292>
        peripherals.CLOCK.tasks_hfclkstart.write(|w| w.bits(1));
        // Wait for the clock to start
        while peripherals.CLOCK.events_hfclkstarted.read().bits() == 0 {}
    }

    // use proprietary radio protocol
    // Source: <https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/source/drivers/MicroBitRadio.cpp#L301>
    peripherals.RADIO.mode.write(|w| w.mode().nrf_1mbit());

    // set power level
    // Source: <https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/source/drivers/MicroBitRadio.cpp#L296>
    peripherals.RADIO.txpower.write(|w| w.txpower()._0d_bm());

    // Source: <https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/source/drivers/MicroBitRadio.cpp#L345>
    peripherals
        .RADIO
        .shorts
        .write(|w| w.ready_start().enabled().end_disable().enabled());

    unsafe {
        peripherals
            .RADIO
            .frequency
            .write(|w| w.frequency().bits(MICROBIT_RADIO_DEFAULT_FREQUENCY)); // 2407 MHz

        // Source: <https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/source/drivers/MicroBitRadio.cpp#L330>
        peripherals.RADIO.crccnf.write(|w| w.len().two());

        peripherals
            .RADIO
            .base0
            .write(|w| w.bits(MICROBIT_RADIO_BASE_ADDRESS));

        peripherals
            .RADIO
            .prefix0
            .write(|w| w.bits(MICROBIT_RADIO_DEFAULT_GROUP));

        peripherals
            .RADIO
            .txaddress
            .write(|w| w.txaddress().bits(MICROBIT_RADIO_DEFAULT_TX_ADDRESS));
        peripherals
            .RADIO
            .rxaddresses
            .write(|w| w.bits(MICROBIT_RADIO_DEFAULT_RX_ADDRESS));

        peripherals
            .RADIO
            .pcnf0
            .write(|w| w.bits(MICROBIT_RADIO_DEFAULT_PCNF0));
        peripherals
            .RADIO
            .pcnf1
            .write(|w| w.bits(MICROBIT_RADIO_DEFAULT_PCNF1 | MICROBIT_RADIO_MAX_PACKET_SIZE));

        peripherals.RADIO.crccnf.write(|w| w.len().two());
        peripherals
            .RADIO
            .crcinit
            .write(|w| w.crcinit().bits(MICROBIT_RADIO_CRCINIT));
        peripherals
            .RADIO
            .crcpoly
            .write(|w| w.crcpoly().bits(MICROBIT_RADIO_CRCPOLY));

        peripherals
            .RADIO
            .datawhiteiv
            .write(|w| w.datawhiteiv().bits(MICROBIT_RADIO_DEFAULT_DATAWHITEIV));

        // Source: <https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/source/drivers/MicroBitRadio.cpp#L341>
        peripherals.RADIO.intenset.write(|w| w.bits(0x0000_0008));

        // Start listening for the next packet
        // Source: <https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/source/drivers/MicroBitRadio.cpp#L348>
        peripherals.RADIO.events_ready.write(|w| w.bits(0));
        peripherals.RADIO.tasks_rxen.write(|w| w.bits(1));
        while peripherals.RADIO.events_ready.read().bits() == 0 {}

        peripherals.RADIO.events_end.write(|w| w.bits(0));
        peripherals.RADIO.tasks_start.write(|w| w.bits(1));
    }

    Ok(())
}

pub fn send(radio: &RADIO, msg: &mut [u8]) -> core::fmt::Result {
    if msg.len() as u32 > MICROBIT_RADIO_MAX_PACKET_SIZE + MICROBIT_RADIO_HEADER_SIZE + 1 {
        return Err(core::fmt::Error);
    }

    // enable "disabled" interrupt
    // Source: <https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/source/drivers/MicroBitRadio.cpp#L509>
    radio.intenset.write(|w| w.disabled().set_bit());

    unsafe {
        // Source: <https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/source/drivers/MicroBitRadio.cpp#L512>
        radio.events_disabled.write(|w| w.bits(0));
        radio.tasks_disable.write(|w| w.bits(1));
        while radio.events_disabled.read().bits() == 0 {}

        // Source: <https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/source/drivers/MicroBitRadio.cpp#L517>
        let p = core::slice::from_raw_parts(msg.as_mut_ptr(), 1);
        defmt::println!("Prepare to send {:#?}", p);
        radio.packetptr.write(|w| w.bits(p.as_ptr() as u32));

        // Source: <https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/source/drivers/MicroBitRadio.cpp#L520>
        radio.events_ready.write(|w| w.bits(0));
        radio.tasks_txen.write(|w| w.bits(1));
        while radio.events_ready.read().bits() == 0 {}

        // Source: <https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/source/drivers/MicroBitRadio.cpp#L525>
        radio.events_end.write(|w| w.bits(0));
        radio.tasks_start.write(|w| w.bits(1));
        while radio.events_end.read().bits() == 0 {}

        // TODO: Receive in a buffer

        // Source: <https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/source/drivers/MicroBitRadio.cpp#L533>
        radio.events_disabled.write(|w| w.bits(0));
        radio.tasks_disable.write(|w| w.bits(1));
        while radio.events_disabled.read().bits() == 0 {}

        // Source: <https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/source/drivers/MicroBitRadio.cpp#L538>
        radio.events_ready.write(|w| w.bits(0));
        radio.tasks_rxen.write(|w| w.bits(1));
        while radio.events_ready.read().bits() == 0 {}

        radio.events_end.write(|w| w.bits(0));
        radio.tasks_start.write(|w| w.bits(1));
    }

    radio.intenset.write(|w| w.ready().set_bit());

    Ok(())
}

pub fn recv(radio: &RADIO, buf: &mut [u8]) -> core::result::Result<(bool, u8), core::fmt::Error> {
    let addr = radio.rxmatch.read().rxmatch().bits();

    unsafe {
        radio.packetptr.write(|w| w.bits(buf.as_mut_ptr() as u32));

        radio.events_ready.write(|w| w.bits(0));
        radio.tasks_rxen.write(|w| w.bits(1));
        while radio.events_ready.read().bits() == 0 {}

        radio.events_end.write(|w| w.bits(0));
        radio.tasks_start.write(|w| w.bits(1));
        while radio.events_end.read().bits() == 0 {}
    }

    let crc_error = radio.crcstatus.read().crcstatus().is_crcerror();

    if crc_error {
        Ok((false, 0))
    } else {
        Ok((true, addr as u8))
    }
}

static mut SEND_BUF: [u8; 1] = *b"1";
static mut RECV_BUF: [u8; 1] = [0; 1];

#[entry]
fn main() -> ! {
    let mut toogle = false;
    if let Some(peripherals) = Peripherals::take() {
        // serial::enable_serial(&peripherals).unwrap();
        enable_radio(&peripherals).unwrap();
        let mut timer = Timer::new(peripherals.TIMER0);

        defmt::println!("Logger works!");
        loop {
            // serial::send(&peripherals.UART0, "Hello, world!\r\n").unwrap();
            // defmt::println!("Sent on UART...");
            // timer.delay_ms(500u32);
            // send(&peripherals.RADIO, b"Hello! micro:rust\r\n").unwrap();
            if toogle {
                unsafe { SEND_BUF = *b"1" };
                toogle = false;
            } else {
                unsafe { SEND_BUF = *b"0" };
                toogle = true;
            }
            // unsafe { send(&peripherals.RADIO, &mut SEND_BUF).unwrap() };
            // defmt::println!("Sent on RADIO...");
            timer.delay_ms(2000u32);
            unsafe { recv(&peripherals.RADIO, &mut RECV_BUF).unwrap() };
            unsafe { defmt::println!("received {:#?}", RECV_BUF) };
        }
    }

    defmt::panic!("End");
}
