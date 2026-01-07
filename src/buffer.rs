
/// UnsafeBuffer implements [`trait core::fmt::Write`]
/// This is used for printing format_args to display.
/// Uses [`from_utf8_unchecked`] so the non ASCII chars from [`characters.rs`] can be printed. 
pub struct UnsafeBuffer<'a> {
    buf : &'a mut[u8],
    len : usize,
}

impl<'a> UnsafeBuffer<'a> {
    #[inline]
    pub fn new(buf:&'a mut [u8]) -> Self {
        Self { buf, len:0 }
    }

    pub fn as_str(&self) -> &str {
        unsafe {
            core::str::from_utf8_unchecked(&self.buf[0..self.len])
        }
    }
}

impl<'a> core::fmt::Write for UnsafeBuffer<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let b = s.as_bytes();
        if self.len + b.len() > self.buf.len() {
            return Err(core::fmt::Error);
        }

        self.buf[self.len..self.len+b.len()].clone_from_slice(b);

        self.len += b.len();
        Ok(())
    }

    fn write_char(&mut self, c: char) -> core::fmt::Result {
        self.buf[self.len] = c as u8;
        self.len += 1;
        
        Ok(())
    }
}