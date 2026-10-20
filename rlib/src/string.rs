use core::fmt;

/// A fixed-capacity UTF-8 string without heap allocation.
pub struct String<const N: usize> {
    // Stores the UTF-8 bytes.
    buffer: [u8; N],

    // Number of bytes currently used.
    len: usize,
}

impl<const N: usize> String<N> {
    /// Creates an empty string.
    pub const fn new() -> Self {
        Self {
            buffer: [0; N],
            len: 0,
        }
    }

    /// Returns the current length in bytes.
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns the maximum capacity in bytes.
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Returns true if the string contains no text.
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the stored text as a string slice.
    pub fn as_str(&self) -> &str {
        // SAFETY: All writes go through methods accepting valid UTF-8.
        unsafe {
            core::str::from_utf8_unchecked(&self.buffer[..self.len])
        }
    }

    /// Clears the string without changing its capacity.
    pub fn clear(&mut self) {
        self.len = 0;
    }

    /// Appends a string slice.
    ///
    /// Returns an error if the string exceeds its capacity.
    pub fn push_str(&mut self, text: &str) -> Result<(), fmt::Error> {
        let bytes = text.as_bytes();

        // Check capacity without risking integer overflow.
        if bytes.len() > N - self.len {
            return Err(fmt::Error);
        }

        // Copy the new UTF-8 bytes into the buffer.
        self.buffer[self.len..self.len + bytes.len()]
            .copy_from_slice(bytes);

        // Update the number of bytes used.
        self.len += bytes.len();

        Ok(())
    }

    /// Appends a single Unicode character.
    pub fn push(&mut self, ch: char) -> Result<(), fmt::Error> {
        // Encode the character into a temporary UTF-8 buffer.
        let mut encoded = [0u8; 4];
        let text = ch.encode_utf8(&mut encoded);

        // Append the encoded character.
        self.push_str(text)
    }
}

impl<const N: usize> Default for String<N> {
    /// Creates an empty string by default.
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> fmt::Write for String<N> {
    /// Allows core::write! to append formatted text.
    fn write_str(&mut self, text: &str) -> fmt::Result {
        self.push_str(text)
    }
}

impl<const N: usize> fmt::Display for String<N> {
    /// Allows the string to be displayed with {}.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl<const N: usize> fmt::Debug for String<N> {
    /// Displays the string with debug formatting.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("String")
            .field(&self.as_str())
            .finish()
    }
}


