#![no_std]
#![no_main]

pub mod drivers;
pub mod arch;

use core::{panic::PanicInfo};

use rlib::{format, string};

use crate::{arch::x86_64::io::outb, drivers::keyboard::scancode::ScancodeDecoder, vga::print_to_vga};

core::arch::global_asm!(include_str!("boot.s"));

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    //hello_world();
    
    //panic!("Test BSOD");

    vga::clear_screen();
    
    let mut decoder = ScancodeDecoder::new();
    loop {
        if let Some(scancode) = drivers::keyboard::ps2::read_scancode() {
            if scancode == 0x0E {
                panic!("Triggered Test BSOD!");
            }
            match decoder.decode(scancode) {
                Some(e) => {
                    vga::clear_screen();
                    vga::print_to_vga(format!("{} - {}",e.key,e.state).as_str());
                },
                None => {}
            }

        }
    }
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let vga = 0xb8000 as *mut u8;

    for i in 0..(80 * 25) {
        unsafe {
            vga.add(i * 2).write_volatile(b' ');
            vga.add(i * 2 + 1).write_volatile(0x1F);
        }
    }

    let (hours, minutes, seconds) = drivers::rtc::read_time();

    let time = [
        b'0' + hours / 10,
        b'0' + hours % 10,
        b':',
        b'0' + minutes / 10,
        b'0' + minutes % 10,
        b':',
        b'0' + seconds / 10,
        b'0' + seconds % 10,
    ];

    let time = core::str::from_utf8(&time).unwrap();

    let msg = rlib::format!("{} => Kernel Panic! {}", time,_info.message().as_str().unwrap());
    for (i, byte) in msg.as_str().bytes().enumerate() {
        unsafe {
            vga.add(i * 2).write_volatile(byte);
            vga.add(i * 2 + 1).write_volatile(0x1F);
        }
    }
    loop {
        unsafe {
            core::arch::asm!("cli; hlt");
        }
    }
}


mod vga;
fn hello_world() {
    vga::clear_screen();
    vga::print_to_vga("Hello World!");
}
