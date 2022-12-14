/*
#[cxx::bridge]
mod ffi_example {
    unsafe extern "C++" {
        include!("../../includes/MicroBit.h");

        pub fn create_microbit();
    }
}
*/

pub mod ffi {
    extern "C" {
        #[link_name = "_Z15create_microbitv"]
        pub fn create_microbit();
    }
}
