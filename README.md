<img align="left" width="110" height="110" src="assets/boo.png">
<p vertical-align="middle"><h1>Boo</h1></p>

[![Crates.io](https://img.shields.io/crates/v/boo-rs)](https://crates.io/crates/boo-rs)
[![Docs.rs](https://docs.rs/boo-rs/badge.svg)](https://docs.rs/boo-rs)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

<br>

Boo encrypts literal data at compile time and keeps it encrypted in the binary, preventing static analysis tools from
reading values.  
At runtime, decrypted data is exposed in memory only for the shortest possible time.

Boo provides compile-time literal encryption beyond strings. It can encrypt numbers, characters, strings, byte strings,
C-strings,
arrays, tuples, nested structures...  
Use the `boo!()` macro around your Rust literals and get them encrypted in the final binary.

## Usage

Add the dependency:

```toml
[dependencies]
boo-rs = "0.1"
```

Set an optional encryption key (or fallback to a 64-byte randomly generated one):

```bash
export BOO_KEY="secret-key"
```

Example:

```rust
extern crate alloc;
#[macro_use]
extern crate boo;

boo_init!();

#[allow(unused_variables)]
fn main() {
    let n = boo!(3);
    let text = boo!("hello");
    let bytes = boo!(b"\x01\x02\x03");
    let pair = boo!(("host", 443));
    let nested = boo!([[1, 2], [3, 4]]);
}
```

`boo_init!()` must be called once before using the `boo!()` macro.
After that, the macro can be used anywhere to encrypt almost all Rust literal values.

Boo supports:

- booleans
- bytes
- integers and floats
- characters
- strings
- byte strings (`b"..."`)
- C-strings (`c"..."`)
- tuples (containing any mix of supported types)
- arrays and nested arrays (containing any of supported types)

For a full reference, see the [showcase](examples/types-showcase.rs) file.

## Performance

Decryption happens on the stack. The cost is O(n), where n is the length of the data in bytes.

All decrypted types except `String` and `CStr` are stored on the stack.    
`&str` and `&CStr` decryption are stored into their heap-allocated variants.

## Roadmap

- [ ] Stack allocated str using a wrapper struct around a fixed `u8` array.
- [ ] Stack allocated Cstr using a wrapper struct around a fixed `u8` array.
- [ ] Numeric suffix support (ex. `1u8`, `1u16`, `0f32`) using `syn::Lit::suffix()`.

## License

MIT - See [LICENSE](LICENSE) for details.

## Credits

* [LITCRYPT](https://github.com/anvie/litcrypt.rs), for being the first crate proposing compile-time string encryption
