#![no_std]
#![no_main]

use cortex_m::asm::wfi;
use teensy4_bsp as bsp;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cortex_m_rt::entry]
unsafe fn main() -> ! {
    let periphs = bsp::Peripherals::take().unwrap();

    let pins = bsp::pins::t41::from_pads(periphs.iomuxc);

    let mut led = bsp::configure_led(pins.p13);
    led.set();
    loop {
        wfi();
    }
}
