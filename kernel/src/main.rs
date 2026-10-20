#![no_std]
#![no_main]

use core::{panic::PanicInfo};

core::arch::global_asm!(include_str!("boot.s"));

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    
    //hello_world();
    
    panic!("Test BSOD");

    loop {
        unsafe {
            core::arch::asm!("");
            core::arch::asm!("cli; hlt");
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

    let msg = rlib::format!("=> Kernel Panic! {}",_info.message().as_str().unwrap());
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
