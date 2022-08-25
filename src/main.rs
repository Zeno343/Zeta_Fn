#![no_std]
#![no_main]
#![feature(default_alloc_error_handler)]

mod hardware;
use hardware::Device;
mod kernel;

#[cortex_m_rt::entry]
unsafe fn main() -> ! {
    kernel::run(&mut Device::init());
}
