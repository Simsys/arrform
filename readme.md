String formatting without memory allocator
==========================================
 
In bare matal systems, there is often the task of converting numbers into text and formatting 
them. The standard Rust functions like format!, write! etc. cannot be used in no_std 
environments because they require a memory allocator. The arrform! macro uses the standard 
library functions, but writes to a fixed length array which is alocated on the stack.

This crate is usable in no_std environments. This is a replacement for the format! macro, based 
on a fixed-size array allocated on the stack.

# arrform!

``` rust
use arrform::{arrform, ArrForm};

let af = arrform!(64, "write some stuff {}: {:.2}", "foo", 42.3456);
assert_eq!("write some stuff foo: 42.35", af.as_str());
```

## ArrForm struct as an alternative

The ArrForm struct provides more detailed error handling and supports multiple use of the 
same buffer. However, it is much more cumbersome to use and generates more syntactic noise. 

# Overhead

The convenient option to format can cost a lot of storage space. On a Cortex M4 992 bytes of 
program code are needed additionally, if instead of a simple string a simple u32 number is 
embedded with the help of the macro. It becomes even more expensive if f32 numbers are output 
formatted (30,928 bytes additional). The program code used to determine these numbers can be 
found in the example directory.

Looking for an alternative that wastes less memory? The Crate [tfmt](https://github.com/Simsys/tfmt) 
can be used as an alternative in most cases. This crate has the additional advantages that it 
is guaranteed not to contain any panic branches and also works much more efficiently.

# License

Apache version 2.0 or Mit
