//! # Warning
//!
//! This module is imported as is in the proc macro.
//! It must be standalone (no dependencies) and support `std` and `no_std` environments.

/// Applies XOR between each byte of `source` and the wrapped `key`.
pub const fn xor(source: &mut [u8], key: &[u8]) {
    if !key.is_empty() {
        let mut i = 0;
        while i < source.len() {
            source[i] = source[i] ^ key[i % key.len()];
            i += 1;
        }
    }
}

/// Splits a `[u8; LEN]` array into two owned arrays `[u8; LEFT]` and `[u8; RIGHT]` without allocating.
#[allow(unused)]
#[inline(always)]
pub const fn split_array<const LEN: usize, const LEFT: usize, const RIGHT: usize>(
    mut arr: [u8; LEN],
) -> ([u8; LEFT], [u8; RIGHT]) {
    // Compile time panic, will be absent at runtime
    const {
        if LEN != LEFT + RIGHT {
            panic!("LEN must be exactly LEFT + RIGHT")
        }
    }

    let ptr = arr.as_mut_ptr();

    // Semantic way is to use `ManuallyDrop` with `deref`, but it is not `const` !
    let _arr = ::core::mem::ManuallyDrop::new(arr);

    // Safety: we assert `LEN == LEFT + RIGHT` above

    let left_ptr = ptr as *mut [u8; LEFT];
    let right_ptr = unsafe { ptr.add(LEFT) } as *mut [u8; RIGHT];

    unsafe { (left_ptr.read(), right_ptr.read()) }
}
