use core::arch::asm;

/// Reads a byte from x86 IO port
pub fn inb(port:u16) -> u8 {
    let v:u8;
    unsafe {
        asm!(
            "in al, dx",
            in("dx") port,
            out("al") v,
            options(nomem, nostack, preserves_flags)
        );
    }
    v
}

/// Writes a byte into x86 IO
pub fn outb(port:u16,value:u8) {
    unsafe {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") value,
            options(nomem, nostack, preserves_flags)
        );
    }
}
