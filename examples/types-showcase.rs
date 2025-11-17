//! Boo Types Showcase
//!
//! Demonstration of all supported types

extern crate alloc;
#[macro_use]
extern crate boo;

boo_init!();

#[allow(unused_variables)]
fn main() {
    // Primitive types
    let max_retries = boo!(3);
    let timeout = boo!(30.5);
    let debug_mode = boo!(false);

    // String types
    let endpoint = boo!("https://im.public.anyway/?");
    let api_key = boo!("secret-api-key");
    let root_password = boo!("ChangeThis!");
    let byte_str = boo!(b"raw\x00bytes");
    let c_str = boo!(c"null terminated");

    // Character types
    let null_char = boo!('\0');
    let newline = boo!('\n');
    let boo = boo!('👻');

    // Byte sequences
    let encryption_key = boo!(b"\x01\x02\x03\x04\x05");
    let magic_bytes = boo!([0x89, 0x50, 0x4E, 0x47]);

    // Collections
    let config_tuple = boo!(("api.example.com", 443, true));
    let mixed_tuple = boo!((42, "answer", b"bytes", '!'));

    // Nested sequences
    let nested_array = boo!([[1, 2, 3], [4, 5, 6]]);
    let complex_structure = boo!((
        "header",
        [0u8, 1, 2],
        [("item1", 3.14), ("item2", 2.718)],
        c"end marker"
    ));
}
