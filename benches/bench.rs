#![feature(test)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
#[macro_use]
extern crate boo;
extern crate test;

use test::Bencher;

boo_init!();

////////////////////////////////////////////////////////////////////////////////
// Literals
////////////////////////////////////////////////////////////////////////////////

#[bench]
fn isize(b: &mut Bencher) {
    b.iter(|| {
        let _ = boo!(0);
    });
}

#[bench]
fn char(b: &mut Bencher) {
    b.iter(|| {
        let _ = boo!('0');
    });
}

/// Heap allocated (slow)
#[bench]
fn str(b: &mut Bencher) {
    b.iter(|| {
        let _ = boo!("Hello World!");
    });
}

#[bench]
fn bstr(b: &mut Bencher) {
    b.iter(|| {
        let _ = boo!(b"Hello World!");
    });
}

/// Heap allocated (slow)
#[bench]
fn cstr(b: &mut Bencher) {
    b.iter(|| {
        let _ = boo!(c"Hello World!");
    });
}

////////////////////////////////////////////////////////////////////////////////
// Vec
////////////////////////////////////////////////////////////////////////////////

#[bench]
fn vec_isize(b: &mut Bencher) {
    b.iter(|| {
        let _ = boo!((1, 2, 3, 4, 5, 6, 7, 8, 9));
    });
}

#[bench]
fn vec_char(b: &mut Bencher) {
    b.iter(|| {
        let _ = boo!(('1', '2', '3', '4', '5', '6', '7', '8', '9'));
    });
}

/// Heap allocated (slow)
#[bench]
fn vec_str(b: &mut Bencher) {
    b.iter(|| {
        let _ = boo!(("1", "2", "3", "4", "5", "6", "7", "8", "9"));
    });
}

#[bench]
fn vec_bstr(b: &mut Bencher) {
    b.iter(|| {
        let _ = boo!((b"1", b"2", b"3", b"4", b"5", b"6", b"7", b"8", b"9"));
    });
}

/// Heap allocated (slow)
#[bench]
fn vec_cstr(b: &mut Bencher) {
    b.iter(|| {
        let _ = boo!((c"1", c"2", c"3", c"4", c"5", c"6", c"7", c"8", c"9"));
    });
}

////////////////////////////////////////////////////////////////////////////////
// Tuple
////////////////////////////////////////////////////////////////////////////////

#[bench]
fn tuple_isize(b: &mut Bencher) {
    b.iter(|| {
        let _ = boo!((1, 2, 3, 4, 5, 6, 7, 8, 9));
    });
}

#[bench]
fn tuple_char(b: &mut Bencher) {
    b.iter(|| {
        let _ = boo!(('1', '2', '3', '4', '5', '6', '7', '8', '9'));
    });
}

/// Heap allocated (slow)
#[bench]
fn tuple_str(b: &mut Bencher) {
    b.iter(|| {
        let _ = boo!(("1", "2", "3", "4", "5", "6", "7", "8", "9"));
    });
}

#[bench]
fn tuple_bstr(b: &mut Bencher) {
    b.iter(|| {
        let _ = boo!((b"1", b"2", b"3", b"4", b"5", b"6", b"7", b"8", b"9"));
    });
}

/// Heap allocated (slow)
#[bench]
fn tuple_cstr(b: &mut Bencher) {
    b.iter(|| {
        let _ = boo!((c"1", c"2", c"3", c"4", c"5", c"6", c"7", c"8", c"9"));
    });
}
