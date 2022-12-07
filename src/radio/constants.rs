//! Source <https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/inc/drivers/MicroBitRadio.h>

// https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/source/drivers/MicroBitRadio.cpp#L308
pub const MICROBIT_RADIO_BASE_ADDRESS: u32 = 0x75626974;

pub const MICROBIT_RADIO_MAX_PACKET_SIZE: u8 = 32;

pub const MICROBIT_RADIO_HEADER_SIZE: u8 = 4;
// https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/inc/drivers/MicroBitRadio.h#L67
pub const MICROBIT_RADIO_DEFAULT_TX_POWER: u8 = 6;

pub const MICROBIT_RADIO_MAXIMUM_RX_BUFFERS: u32 = 4;
// https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/source/drivers/MicroBitRadio.cpp#L331
pub const MICROBIT_RADIO_CRCINIT: u32 = 0x0000_FFFF;
// https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/source/drivers/MicroBitRadio.cpp#L332
pub const MICROBIT_RADIO_CRCPOLY: u32 = 0x0001_1021;
// https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/inc/core/MicroBitConfig.h#L320
pub const MICROBIT_RADIO_DEFAULT_FREQUENCY: u8 = 7;
