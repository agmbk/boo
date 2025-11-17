#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
#[macro_use]
extern crate boo;

use alloc::borrow::ToOwned;

boo_init!();

#[test]
fn test_literal_bool() {
    assert!(boo!(true));
    assert!(!boo!(false));
}

#[test]
fn test_literal_isize() {
    assert_eq!(boo!(-9223372036854775808), -9223372036854775808);

    assert_eq!(boo!(-1), -1);

    assert_eq!(boo!(-0), -0);

    assert_eq!(boo!(0), 0);

    assert_eq!(boo!(1), 1);

    assert_eq!(boo!(9223372036854775807), 9223372036854775807);
}

#[test]
fn test_literal_f64() {
    assert_eq!(
        boo!(-1.7976931348623157e+308_f64),
        -1.7976931348623157e+308_f64
    );

    assert_eq!(boo!(-1.), -1.);

    assert_eq!(boo!(0.), 0.);
    assert_eq!(boo!(0.0).is_sign_positive(), 0f64.is_sign_positive());
    assert_eq!(boo!(-0.0).is_sign_positive(), (-0f64).is_sign_positive());

    assert_eq!(boo!(1.), 1.);

    assert_eq!(boo!(1.7976931348623157e+308), 1.7976931348623157e+308);
}

#[test]
fn test_literal_char() {
    assert_eq!(boo!('\0'), '\0');
    assert_eq!(boo!('\n'), '\n');
    assert_eq!(boo!('\\'), '\\');

    assert_eq!(boo!('0'), '0');
    assert_eq!(boo!('9'), '9');

    assert_eq!(boo!('a'), 'a');
    assert_eq!(boo!('z'), 'z');
    assert_eq!(boo!('A'), 'A');
    assert_eq!(boo!('Z'), 'Z');

    assert_eq!(boo!('\u{10AAAA}'), '\u{10AAAA}');
}

#[test]
fn test_literal_byte() {
    assert_eq!(boo!(b'\0'), b'\0');
    assert_eq!(boo!(b'\n'), b'\n');
    assert_eq!(boo!(b'\\'), b'\\');

    assert_eq!(boo!(b'0'), b'0');
    assert_eq!(boo!(b'9'), b'9');

    assert_eq!(boo!(b'a'), b'a');
    assert_eq!(boo!(b'z'), b'z');
    assert_eq!(boo!(b'A'), b'A');
    assert_eq!(boo!(b'Z'), b'Z');
}

#[test]
fn test_literal_str() {
    assert_eq!(boo!("\0\n\\ 09azAZ \u{10FFFF}"), "\0\n\\ 09azAZ \u{10FFFF}");

    assert_eq!(boo!(r"raw"), r"raw");
    assert_eq!(boo!(r#"raw"#), r#"raw"#);
    assert_eq!(
        boo!(
            r#"
        multiline
        "#
        ),
        r#"
        multiline
        "#
    );
}

#[test]
fn test_literal_bstr() {
    assert_eq!(boo!(b"\0\n\\ 09azAZ"), b"\0\n\\ 09azAZ".to_owned());

    assert_eq!(boo!(br"raw"), br"raw".to_owned());
    assert_eq!(boo!(br#"raw"#), br#"raw"#.to_owned());
    assert_eq!(
        boo!(
            br#"
        multiline
        "#
        ),
        br#"
        multiline
        "#
        .to_owned()
    );
}

#[test]
fn test_literal_cstr() {
    assert_eq!(boo!(c"\n\\ 09azAZ \u{10FFFF}"), c"\n\\ 09azAZ \u{10FFFF}");

    assert_eq!(boo!(cr"raw"), cr"raw");
    assert_eq!(boo!(cr#"raw"#), cr#"raw"#);
    assert_eq!(
        boo!(
            cr#"
        multiline
        "#
        ),
        cr#"
        multiline
        "#
    );
}

#[test]
fn test_literal_array_bool() {
    assert_eq!(boo!([true, false]), [true, false]);
}

#[test]
fn test_literal_array_u8() {
    assert_eq!(boo!([0, 1, 2, 3, 255]), [0, 1, 2, 3, 255]);
}

#[test]
fn test_literal_array_float() {
    assert_eq!(boo!([0., 1., 2., 3., 255.]), [0., 1., 2., 3., 255.]);
}

#[test]
fn test_literal_array_char() {
    assert_eq!(boo!(['0', '1', '2', '3']), ['0', '1', '2', '3']);
}

#[test]
fn test_literal_array_str() {
    assert_eq!(boo!(["\0", "b", "c", ""]), ["\0", "b", "c", ""]);
}

#[test]
fn test_literal_array_bstr() {
    assert_eq!(
        boo!([b"\012", b"bbb", b"ccc", b"abc"]),
        [
            b"\012".to_owned(),
            b"bbb".to_owned(),
            b"ccc".to_owned(),
            b"abc".to_owned()
        ]
    );
}

#[test]
fn test_literal_array_cstr() {
    assert_eq!(boo!([c"a", c"b", c"c", c""]), [c"a", c"b", c"c", c""]);
}

#[test]
fn test_literal_bool_sequence() {
    assert_eq!(boo!([true, false]), [true, false]);
    assert_eq!(boo!([[true], [false]]), [[true], [false]]);
    assert_eq!(boo!((true, false)), (true, false));
    assert_eq!(boo!((true, (false, true))), (true, (false, true)));
}

#[test]
fn test_literal_isize_sequence() {
    assert_eq!(
        boo!([-9223372036854775808, -1, 0, 1, 9223372036854775807]),
        [-9223372036854775808, -1, 0, 1, 9223372036854775807]
    );
    assert_eq!(boo!([[1], [2], [3]]), [[1], [2], [3]]);
    assert_eq!(boo!((1, 2, 3)), (1, 2, 3));
    assert_eq!(boo!((1, (2, 3))), (1, (2, 3)));
}

#[test]
fn test_literal_f64_sequence() {
    assert_eq!(
        boo!([
            -1.7976931348623157e+308,
            -1.0,
            -0.0,
            0.0,
            1.0,
            1.7976931348623157e+308
        ]),
        [
            -1.7976931348623157e+308,
            -1.0,
            -0.0,
            0.0,
            1.0,
            1.7976931348623157e+308
        ]
    );
    assert_eq!(boo!([[0.0], [1.0]]), [[0.0], [1.0]]);
    assert_eq!(boo!((0.0, 1.0)), (0.0, 1.0));
    assert_eq!(boo!((0.0, (1.0, -1.0))), (0.0, (1.0, -1.0)));
}

#[test]
fn test_literal_char_sequence() {
    assert_eq!(
        boo!(['a', 'z', 'A', 'Z', '\0', '\n', '\\']),
        ['a', 'z', 'A', 'Z', '\0', '\n', '\\']
    );
    assert_eq!(boo!([['0'], ['1']]), [['0'], ['1']]);
    assert_eq!(boo!(('a', 'Z')), ('a', 'Z'));
    assert_eq!(boo!(('a', ('b', 'c'))), ('a', ('b', 'c')));
}

#[test]
fn test_literal_byte_sequence() {
    assert_eq!(
        boo!([b'a', b'z', b'A', b'Z', b'\0', b'\n', b'\\']),
        [b'a', b'z', b'A', b'Z', b'\0', b'\n', b'\\']
    );
    assert_eq!(boo!([[b'0'], [b'1']]), [[b'0'], [b'1']]);
    assert_eq!(boo!((b'a', b'Z')), (b'a', b'Z'));
    assert_eq!(boo!((b'a', ('b', b'c'))), (b'a', ('b', b'c')));
}

#[test]
fn test_literal_str_sequence() {
    assert_eq!(
        boo!(["\0", "\n", "\\", "abc", "ABC", "\u{10FFFF}"]),
        ["\0", "\n", "\\", "abc", "ABC", "\u{10FFFF}"]
    );
    assert_eq!(boo!([["a"], ["b"]]), [["a"], ["b"]]);
    assert_eq!(boo!(("a", "b")), ("a".to_owned(), "b".to_owned()));
    assert_eq!(
        boo!(("a", ("b", "c"))),
        ("a".to_owned(), ("b".to_owned(), "c".to_owned()))
    );
}

#[test]
fn test_literal_bstr_sequence() {
    assert_eq!(
        boo!([b"abc", b"\n\\ ", b"\012"]),
        [b"abc".to_owned(), b"\n\\ ".to_owned(), b"\012".to_owned()]
    );
    assert_eq!(
        boo!([[b"a"], [b"b"]]),
        [[b"a".to_owned()], [b"b".to_owned()]]
    );
    assert_eq!(boo!((b"a", b"b")), (b"a".to_owned(), b"b".to_owned()));
    assert_eq!(
        boo!((b"a", (b"b", b"c"))),
        (b"a".to_owned(), (b"b".to_owned(), b"c".to_owned()))
    );
}

#[test]
fn test_literal_cstr_sequence() {
    assert_eq!(
        boo!([c"abc", c"\n\\ ", c"\u{10FFFF}"]),
        [c"abc", c"\n\\ ", c"\u{10FFFF}"]
    );
    assert_eq!(boo!([[c"a"], [c"b"]]), [[c"a"], [c"b"]]);
    assert_eq!(boo!((c"a", c"b")), (c"a".to_owned(), c"b".to_owned()));
    assert_eq!(
        boo!((c"a", (c"b", c"c"))),
        (c"a".to_owned(), (c"b".to_owned(), c"c".to_owned()))
    );
}

#[test]
fn test_include_bytes() {
    assert_eq!(
        &boo_include_bytes!("../assets/lorem_ipsum.txt"),
        b"Lorem ipsum dolor sit amet"
    );
}

#[test]
fn test_include_str() {
    assert_eq!(
        boo_include_str!("../assets/lorem_ipsum.txt"),
        "Lorem ipsum dolor sit amet"
    );
}
