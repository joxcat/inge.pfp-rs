use microbit::Peripherals;

use crate::pin_names::*;

pub fn enable_serial(peripherals: &Peripherals) -> core::fmt::Result {
    peripherals.GPIO.pin_cnf[USBTX as usize].write(|w| w.pull().pullup().dir().output());
    peripherals.GPIO.pin_cnf[USBRX as usize].write(|w| w.pull().disabled().dir().input());

    peripherals
        .UART0
        .pseltxd
        .write(|w| unsafe { w.bits(USBTX) });
    peripherals
        .UART0
        .pselrxd
        .write(|w| unsafe { w.bits(USBRX) });

    peripherals
        .UART0
        .baudrate
        .write(|w| w.baudrate().baud115200());
    peripherals.UART0.enable.write(|w| w.enable().enabled());
    Ok(())
}

pub fn send(uart0: &microbit::pac::UART0, msg: &str) -> core::fmt::Result {
    uart0.tasks_starttx.write(|w| unsafe { w.bits(1) });
    for c in msg.as_bytes() {
        /* Write the current character to the output register */
        uart0.txd.write(|w| unsafe { w.bits(u32::from(*c)) });

        /* Wait until the UART is clear to send */
        while uart0.events_txdrdy.read().bits() == 0 {}

        /* Clear the event */
        uart0.events_txdrdy.write(|w| unsafe { w.bits(0) });
    }
    uart0.tasks_stoptx.write(|w| unsafe { w.bits(1) });
    Ok(())
}
