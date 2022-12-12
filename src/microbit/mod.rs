#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("../../includes/ManagedString.h");
        include!("../../includes/MicroBit.h");

        pub type Microbit;

        // pub fn new_microbit() -> UniquePtr<Microbit>;
        // pub fn init(m: &UniquePtr<Microbit>);
    }
}
