#![no_std]
#![no_main]

use core::fmt::Write;

#[repr(u8)]
enum Ascii {
    Space = 32,
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cortex_m_rt::entry]
unsafe fn main() -> ! {
    let mut periphs = bsp::Peripherals::take().unwrap();
    let pins = bsp::pins::t41::from_pads(periphs.iomuxc);
    let mut led = bsp::configure_led(pins.p13);
    let mut systick = systick::new(cortex_m::Peripherals::take().unwrap().SYST);

    let (mut reader, mut writer) = usb::split().unwrap();
    systick.delay_ms(1_000);
    periphs.ccm.pll1.set_arm_clock(
        bsp::hal::ccm::PLL1::ARM_HZ,
        &mut periphs.ccm.handle,
        &mut periphs.dcdc,
    );

    let mut x = 0u8;
    while x < 3 {
        use embedded_hal::digital::v2::OutputPin;

        led.set_high().unwrap();
        systick.delay_ms(30);
        led.set_low().unwrap();
        systick.delay_ms(30);
        x += 1;
    }

    let mut usb_in = [0; 256];
    loop {
        if let Ok(n) = reader.read(&mut usb_in) {
            if n == 1 && usb_in[0] == Ascii::Space as u8 {
                break;
            }
        } else {
            panic!();
        }
    }

    let mut counter = 0u8;
    loop {
        writeln!(writer, "{}", counter).unwrap();
        writer.flush().unwrap();

        systick.delay_ms(1_000);
        counter += 1;
    }
}

mod systick {
    use cortex_m::{
        delay::Delay,
        peripheral::{syst::SystClkSource, SYST},
    };

    pub type SysTick = Delay;

    pub fn new(syst: SYST) -> SysTick {
        Delay::with_source(syst, bsp::EXT_SYSTICK_HZ, SystClkSource::External)
    }
}

mod usb {
    use core::cell::RefCell;

    use bsp::hal::ral::usb::USB1;
    use bsp::interrupt;
    use cortex_m::interrupt::Mutex;

    pub fn split() -> Result<(bsp::usb::Reader, bsp::usb::Writer), bsp::usb::Error> {
        let inst = USB1::take().unwrap();
        bsp::usb::split(inst).map(|(poller, reader, writer)| {
            setup(poller);
            (reader, writer)
        })
    }

    fn setup(poller: bsp::usb::Poller) {
        static POLLER: Mutex<RefCell<Option<bsp::usb::Poller>>> = Mutex::new(RefCell::new(None));

        #[cortex_m_rt::interrupt]
        fn USB_OTG1() {
            cortex_m::interrupt::free(|cs| {
                POLLER
                    .borrow(cs)
                    .borrow_mut()
                    .as_mut()
                    .map(|poller| poller.poll());
            });
        }

        cortex_m::interrupt::free(|cs| {
            *POLLER.borrow(cs).borrow_mut() = Some(poller);
            unsafe { cortex_m::peripheral::NVIC::unmask(bsp::interrupt::USB_OTG1) };
        });
    }
}
