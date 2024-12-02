#![no_std]

//! String formatting without memory allocator
//! ==========================================
//! 
//! In bare metal systems, there is often the task of converting numbers into text and formatting 
//! them. The standard Rust functions like format!, write! etc. cannot be used in no_std 
//! environments because they require a memory allocator. The arrform! macro uses the standard 
//! library functions, but writes to a fixed length array which is alocated on the stack.
//! 
//! This crate is usable in no_std environments. This is a replacement for the format! macro, based 
//! on a fixed-size array allocated on the stack.
//! 
//! # arrform!
//! 
//! ``` rust
//! use arrform::arrform;
//! 
//! let af = arrform!(64, "write some stuff {}: {:.2}", "foo", 42.3456);
//! assert_eq!("write some stuff foo: 42.35", af.as_str());
//! ```
//! 
//! ## ArrForm struct as an alternative
//! 
//! The [ArrForm] struct provides more detailed error handling and supports multiple use of the 
//! same buffer. However, it is much more cumbersome to use and generates more syntactic noise. 
//! 
//! # Overhead
//! 
//! The convenient option to format can cost a lot of storage space. On a Cortex M4 992 bytes of 
//! program code are needed additionally, if instead of a simple string a simple u32 number is 
//! embedded with the help of the macro. It becomes even more expensive if f32 numbers are output 
//! formatted (30,928 bytes additional). The program code used to determine these numbers can be 
//! found in the example directory.
//! 
//! # License
//! 
//! Apache version 2.0 or Mit
//!
use core::{fmt, str::from_utf8_unchecked};
use core::mem::MaybeUninit;

#[allow(unused_imports)]
use core::format_args;

/// Generates formatted text in a buffer on the stack
/// 
/// Allows precise handling of errors. A buffer created once can be used several times. The 
/// application requires more typing and contains some syntactic noise.
/// ```
/// use arrform::ArrForm;
/// 
/// let mut af = ArrForm::<64>::new();
/// match af.format(format_args!("write some stuff {}: {:.2}", "foo", 42.3456)) {
///     Ok(()) => {
///         assert_eq!("write some stuff foo: 42.35", af.as_str());
///         assert_eq!(b"write some stuff foo: 42.35", af.as_bytes());
///     },
///     Err(_) => {
///         panic!("An error occurred");
///     }
/// }
/// 
/// // Use the buffer a second time
/// af.format(
///     format_args!("same buffer, new {}, int {}, float {:.1}", "text", 123, 4.1234)
/// ).unwrap();
/// 
/// assert_eq!("same buffer, new text, int 123, float 4.1", af.as_str());
/// ```
pub struct ArrForm<const BUF_SIZE: usize> {
    buffer: [u8; BUF_SIZE],
    used: usize,
}

impl<const BUF_SIZE: usize> ArrForm<BUF_SIZE> {

    /// Creates new buffer on the stack
    pub fn new() -> Self {
        // We don't need to initialize, because we write before we read
        let buffer: [u8; BUF_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
        ArrForm { buffer, used: 0 }
    }

    /// Format numbers and strings
    pub fn format(&mut self, args: fmt::Arguments) -> fmt::Result {
        self.used = 0;                  // if format is used several times
        fmt::write(self, args)
    }

    /// Get a reference to the result as a slice inside the buffer as str
    pub fn as_str(&self) -> &str {
        // We are really sure, that the buffer contains only valid utf8 characters
        unsafe { from_utf8_unchecked(&self.buffer[..self.used]) }
    }

    /// Get a reference to the result as a slice inside the buffer as bytes
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

/// A macro to format numbers into text, based on a fixed-size array allocated on the stack
/// 
/// This macro first reserves a buffer on the stack. Then it uses the struct [ArrForm] to format 
/// text and numbers. It returns an instance of ArrForm that allows easy access to the contained 
/// text. The macro panics if the buffer is chosen too small.
/// 
/// ```
/// use arrform::arrform;
/// 
/// let af = arrform!(64, "write some {}, int {}, float {:.3}", "stuff", 4711, 3.1415);
/// assert_eq!("write some stuff, int 4711, float 3.142", af.as_str());
/// ```
#[macro_export]
macro_rules! arrform {
    ($size:expr, $($arg:tt)*) => {{
        let mut af = $crate::ArrForm::<$size>::new();

        // Panic on buffer overflow
        af.format(format_args!($($arg)*)).expect("Buffer overflow");
        af
    }}
}
