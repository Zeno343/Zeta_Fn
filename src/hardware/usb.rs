pub use bsp::usb::{Error, Poller, Reader, Writer};
const BUFFER_LEN: usize = 8;
pub struct Usb1 {
    reader: bsp::usb::Reader,
    writer: bsp::usb::Writer,
    buf: [u8; BUFFER_LEN],
}

impl Usb1 {
    pub unsafe fn init() -> Result<Usb1, Error> {
        use bsp::{hal::ral::usb::USB1, usb::split};

        let inst = USB1::take().unwrap();
        split(inst)
            .map(|(poller, reader, writer)| {
                setup(poller);
                (reader, writer)
            })
            .map(|(reader, writer)| Usb1 {
                reader,
                writer,
                buf: [0; BUFFER_LEN],
            })
    }

    pub fn read(&mut self) -> Result<&[u8], Error> {
        self.reader
            .read(&mut self.buf)
            .map(|n_bytes| &self.buf[0..n_bytes])
    }

    pub fn write(&mut self, bytes: &[u8]) -> Result<usize, Error> {
        self.writer.write(bytes)
    }
}

fn setup(poller: Poller) {
    use core::cell::RefCell;

    use bsp::interrupt;
    use cortex_m::{
        interrupt::{free, Mutex},
        peripheral::NVIC,
    };

    static POLLER: Mutex<RefCell<Option<Poller>>> = Mutex::new(RefCell::new(None));

    // Pause interrupts while we prepare the Usb1 ISR
    free(|cs| unsafe {
        *POLLER.borrow(cs).borrow_mut() = Some(poller);
        NVIC::unmask(bsp::interrupt::USB_OTG1);

        #[cortex_m_rt::interrupt]
        fn USB_OTG1() {
            free(|cs| {
                POLLER
                    .borrow(cs)
                    .borrow_mut()
                    .as_mut()
                    .map(|poller| poller.poll());
            });
        }
    });
}
