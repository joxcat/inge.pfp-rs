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
    use core::ffi::c_int;

    extern "C" {
        #[link_name = "create_microbit"]
        pub fn create_microbit() -> c_int;
        #[link_name = "create_serial"]
        pub fn create_serial() -> c_int;

        #[link_name = "ping"]
        pub fn ping() -> c_int;
    }
}
