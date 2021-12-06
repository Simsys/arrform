#![no_std]

use core::{fmt, str::from_utf8_unchecked};
use core::mem::MaybeUninit;

#[allow(unused_imports)]
use core::format_args;

pub struct ArrForm<const BUF_SIZE: usize> {
    buffer: [u8; BUF_SIZE],
    used: usize,
}

impl<const BUF_SIZE: usize> ArrForm<BUF_SIZE> {

    pub fn new() -> Self {
        // We don't need to initialize, because we write before we read
        let buffer: [u8; BUF_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
        ArrForm { buffer, used: 0 }
    }

    pub fn format(&mut self, args: fmt::Arguments) -> fmt::Result {
        self.used = 0;                  // if format is used several times
        fmt::write(self, args)
    }

    pub fn as_str(&self) -> &str {
        // We are really sure, that the buffer contains only valid utf8 characters
        unsafe { from_utf8_unchecked(&self.buffer[..self.used]) }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.buffer[..self.used]
    }
}

impl<const BUF_SIZE: usize> fmt::Write for ArrForm<BUF_SIZE> {

    fn write_str(&mut self, s: &str) -> fmt::Result {
        let remaining_buf = &mut self.buffer[self.used..];
        let raw_s = s.as_bytes();

        // Treat imminent buffer overflow
        if raw_s.len() > remaining_buf.len() {
            remaining_buf.copy_from_slice(&raw_s[..remaining_buf.len()]);
            self.used += remaining_buf.len();
            Err(fmt::Error)
        } else {
            remaining_buf[..raw_s.len()].copy_from_slice(raw_s);
            self.used += raw_s.len();
            Ok(())
        }
    }
}

#[macro_export]
macro_rules! arrform {
    ($size:expr, $($arg:tt)*) => {{
        let mut af = ArrForm::<$size>::new();

        // Panic on buffer overflow
        af.format(format_args!($($arg)*)).expect("Buffer overflow");
        af
    }}
}

#[test]
fn format_macro() {
    let s = arrform!(64, "write some stuff {}: {:.2}", "foo", 42.3456);
    assert_eq!("write some stuff foo: 42.35", s.as_str());
    assert_eq!(b"write some stuff foo: 42.35", s.as_bytes());
}

#[test]
fn format_struct() {
    let mut s2 = ArrForm::<64>::new();
    match s2.format(format_args!("write some stuff {}: {:.2}", "foo", 42.3456)) {
        Ok(()) => {
            assert_eq!("write some stuff foo: 42.35", s2.as_str());
            assert_eq!(b"write some stuff foo: 42.35", s2.as_bytes());
        },
        Err(_) => {
            panic!("An error occurred");
        }
    }
}
