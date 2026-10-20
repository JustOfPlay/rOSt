



pub fn print_to_vga(_string: &str) {
    let vga = 0xb8000 as *mut u8;

    for (i, byte) in _string.bytes().enumerate() {
        unsafe {
            vga.add(i * 2).write_volatile(byte);
            vga.add(i * 2 + 1).write_volatile(0x0f);
        }
    }
}

pub fn clear_screen() {
    let vga = 0xb8000 as *mut u8;

    for i in 0..(80 * 25) {
        unsafe {
            vga.add(i * 2).write_volatile(b' ');
            vga.add(i * 2 + 1).write_volatile(0x0f);
        }
    }
}
