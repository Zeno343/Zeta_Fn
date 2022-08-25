extern crate alloc;
use alloc::format;

use alloc_cortex_m::CortexMHeap;
#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

use crate::hardware::Device;
pub unsafe fn run(dev: &mut Device) -> ! {
    use core::mem::MaybeUninit;

    // Set up the heap
    const HEAP_SIZE: usize = 1024;
    static mut HEAP: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    ALLOCATOR.init(HEAP.as_ptr() as usize, HEAP_SIZE);

    // Blink 3 times if you can hear me
    dev.blink(3, 30, 30).unwrap();

    // Get USB1 and wait for a signal
    dev.delay_ms(1_000);
    let usb = dev.usb1().unwrap();

    usb.write(b"Zeta_OS v0.1.0\r\n").unwrap();
    loop {
        let bytes = usb.read().unwrap();
        if bytes.len() > 0 {
            break;
        }
    }

    let mut counter = 0u8;
    loop {
        usb.write(format!("{}\r\n", counter).as_bytes()).unwrap();
        if counter >= 9 {
            panic!();
        } else {
            counter += 1;
            dev.delay_ms(1_000);
        }
    }
}
