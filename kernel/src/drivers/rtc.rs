
use crate::arch::x86_64::io::{inb, outb};

// cmos register addresses
const CMOS_SECONDS: u8 = 0x00;
const CMOS_MINUTES: u8 = 0x02;
const CMOS_HOURS: u8 = 0x04;
const CMOS_STATUS_A: u8 = 0x0A;
const CMOS_STATUS_B: u8 = 0x0B;

// Select a cmos register and read its value
unsafe fn read_register(register: u8) -> u8 {
    unsafe {
        outb(0x70, register);

        inb(0x71)
    }
}

// Convert a binary decimal into a normal integer
fn bcd_to_binary(value: u8) -> u8 {
    (value >> 4) * 10 + (value & 0x0F)
}

// Read the current time as (hms)
pub fn read_time() -> (u8, u8, u8) {
    unsafe {
        while read_register(CMOS_STATUS_A) & 0x80 != 0 {}

        let status_b = read_register(CMOS_STATUS_B);

        // Bit 2 for binary mode else values use BCD
        let binary_mode = status_b & 0x04 != 0;

        // Bit 1 for 24 hour mode
        let mode_24h = status_b & 0x02 != 0;

        // Read the raw time values from the CMOS
        let seconds_raw = read_register(CMOS_SECONDS);
        let minutes_raw = read_register(CMOS_MINUTES);
        let hours_raw = read_register(CMOS_HOURS);

        // In 12hour mode bit 7 for PM
        let is_pm = !mode_24h && hours_raw & 0x80 != 0;
        let hours_raw = hours_raw & 0x7F;

        // Convert BCD values when the RTC is not in binary mode
        let seconds = if binary_mode {
            seconds_raw
        } else {
            bcd_to_binary(seconds_raw)
        };

        let minutes = if binary_mode {
            minutes_raw
        } else {
            bcd_to_binary(minutes_raw)
        };

        let mut hours = if binary_mode {
            hours_raw
        } else {
            bcd_to_binary(hours_raw)
        };

        // Convert 12 hour time into 24hour
        if !mode_24h {
            hours %= 12;

            if is_pm {
                hours += 12;
            }
        }

        (hours, minutes, seconds)
    }
}
