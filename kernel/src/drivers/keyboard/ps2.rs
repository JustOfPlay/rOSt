use crate::arch::x86_64::io::inb;



pub fn read_scancode() -> Option<u8> {
    let status = inb(0x64);

    if status & 1 == 0 {
        return None;
    }

    if status & 0xC0 != 0 {
        let _ = inb(0x60);
        return None;
    }

    Some(inb(0x60))
}
