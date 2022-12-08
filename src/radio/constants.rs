//! Source <https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/inc/drivers/MicroBitRadio.h>

// Source: <https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/source/drivers/MicroBitRadio.cpp#L308>
pub const MICROBIT_RADIO_BASE_ADDRESS: u32 = 0x7562_6974;
// Source: <https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/inc/drivers/MicroBitRadio.h#L68>
pub const MICROBIT_RADIO_MAX_PACKET_SIZE: u32 = 32;
// Source: <https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/inc/drivers/MicroBitRadio.h#L66>
pub const MICROBIT_RADIO_DEFAULT_GROUP: u32 = 0;
// Source: <https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/inc/drivers/MicroBitRadio.h#L69>
pub const MICROBIT_RADIO_HEADER_SIZE: u32 = 4;
// Source: <https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/inc/drivers/MicroBitRadio.h#L70>
pub const MICROBIT_RADIO_MAXIMUM_RX_BUFFERS: u32 = 4; // TODO: Unused
                                                      // Source: <https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/source/drivers/MicroBitRadio.cpp#L331>
pub const MICROBIT_RADIO_CRCINIT: u32 = 0xFFFF;
// Source: <https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/source/drivers/MicroBitRadio.cpp#L332>
pub const MICROBIT_RADIO_CRCPOLY: u32 = 0x11021;
// Source: <https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/inc/core/MicroBitConfig.h#L320>
pub const MICROBIT_RADIO_DEFAULT_FREQUENCY: u8 = 7;
// Source: <https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/source/drivers/MicroBitRadio.cpp#L315>
pub const MICROBIT_RADIO_DEFAULT_TX_ADDRESS: u8 = 0;
// Source: <https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/source/drivers/MicroBitRadio.cpp#L316>
pub const MICROBIT_RADIO_DEFAULT_RX_ADDRESS: u32 = 1;
// Source: <https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/source/drivers/MicroBitRadio.cpp#L321>
pub const MICROBIT_RADIO_DEFAULT_PCNF0: u32 = 0x0000_0000;
// Source: <https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/source/drivers/MicroBitRadio.cpp#L322>
pub const MICROBIT_RADIO_DEFAULT_PCNF1: u32 = 0x0204_0000;
// Source: <https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/source/drivers/MicroBitRadio.cpp#L335>
pub const MICROBIT_RADIO_DEFAULT_DATAWHITEIV: u8 = 0x18;
