mod usb;
pub use bsp::Led;
pub use usb::Usb1;

pub struct Device(*mut DevInterface);
static mut DEVICE: Device = Device(core::ptr::null_mut());

#[derive(Default)]
pub struct DevInterface {
    led: Option<Led>,
    usb1: Option<Usb1>,
}

impl Device {
    pub unsafe fn init() -> &'static mut Self {
        use bsp::{configure_led, hal::ccm::PLL1, pins::t41::from_pads, Peripherals};
        static mut DEV_INT: DevInterface = DevInterface {
            led: None,
            usb1: None,
        };

        // Get Teensy4.1 pinout info
        let mut periphs = Peripherals::steal();
        let pins = from_pads(periphs.iomuxc);

        // Init peripherals
        DEV_INT.led = Some(configure_led(pins.p13));
        DEV_INT.usb1 = Usb1::init().ok();

        // Configure clock
        periphs
            .ccm
            .pll1
            .set_arm_clock(PLL1::ARM_HZ, &mut periphs.ccm.handle, &mut periphs.dcdc);

        DEVICE = Device(&mut DEV_INT);
        &mut DEVICE
    }

    pub unsafe fn blink(&mut self, times: usize, pw: u32, delay: u32) -> Result<(), DeviceErr> {
        match (*DEVICE.0).led.as_mut() {
            Some(led) => {
                for _ in 0..times {
                    led.set();
                    self.delay_ms(pw);
                    led.clear();
                    self.delay_ms(delay);
                }

                Ok(())
            }

            None => Err(DeviceErr::UnInit),
        }
    }

    pub unsafe fn delay_ms(&self, ms: u32) {
        use bsp::EXT_SYSTICK_HZ;
        use cortex_m::{delay::Delay, peripheral::syst::SystClkSource, Peripherals};

        Delay::with_source(
            Peripherals::steal().SYST,
            EXT_SYSTICK_HZ,
            SystClkSource::External,
        )
        .delay_ms(ms);
    }

    pub unsafe fn usb1(&self) -> Option<&mut Usb1> {
        (*self.0).usb1.as_mut()
    }
}

#[derive(Debug)]
pub enum DeviceErr {
    UnInit,
}

#[panic_handler]
unsafe fn panic(_info: &core::panic::PanicInfo) -> ! {
    const SHORT: u32 = 10;
    const LONG: u32 = 120;
    const GAP: u32 = 60;

    let s = || {
        DEVICE.blink(3, SHORT, LONG + GAP).unwrap();
    };

    let o = || {
        DEVICE.blink(3, LONG, SHORT + GAP).unwrap();
    };

    loop {
        s();
        o();
        s();

        DEVICE.delay_ms(3 * (SHORT + LONG + GAP));
    }
}
