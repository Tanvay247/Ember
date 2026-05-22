#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

use bootloader_api::{entry_point, BootInfo};
use core::fmt::Write;
use core::panic::PanicInfo;
use core::sync::atomic::{AtomicBool, Ordering};
use uart_16550::SerialPort;
use x86_64::instructions::interrupts as x86_interrupts;

mod gdt;
mod interrupts;
#[macro_use]
mod serial;

static PANICKED: AtomicBool = AtomicBool::new(false);

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    gdt::init();
    interrupts::init_idt();

    println!("[BOOT OK]");
    println!("Ember kernel initialized");

    x86_interrupts::int3();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    if PANICKED.swap(true, Ordering::Relaxed) {
        loop {
            core::hint::spin_loop();
        }
    }

    let mut serial_port = unsafe {
        SerialPort::new(0x3F8)
    };

    serial_port.init();

    writeln!(serial_port, "[KERNEL PANIC]").ok();

    loop {
        core::hint::spin_loop();
    }
}