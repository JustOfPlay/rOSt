#![no_std]

pub mod collections;

pub mod string;


/// Formats arguments into a fixed-capacity String without heap allocation.
///
/// Example: format!("Frame: {}", 42)
#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {{
        // Import the formatting trait required by write!.
        use core::fmt::Write as _;

        // Create a string with a capacity of 1024 bytes.
        let mut output = $crate::string::String::<1024>::new();

        // Format the arguments into the string.
        core::write!(&mut output, $($arg)*)
            .expect("format! buffer capacity exceeded");

        // Return the owned string.
        output
    }};
}
