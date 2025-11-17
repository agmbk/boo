use std::borrow::Cow;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Expr, ExprArray, ExprTuple, ExprUnary, Lit, UnOp};

use crate::utils::xor;
use crate::KEY;

/// Global error message with usage hints
const GLOBAL_ERROR: &str = r#"expected one literal parameter (like `true`, `1`, `1.14`, `'d'`, `b'e'`, `"foo"`, `b"bar"`, `c"baz"`, `["1", "2", "3"]`, `(99, "20", [3.3, 4.5])`)"#;

/// Bytes representation of a Rust literal
pub enum LiteralBytes {
    /// A UTF-8 string literal: `"foo"`.
    Str(Vec<u8>),

    /// A byte string literal: `b"foo"`.
    ByteStr(Vec<u8>),

    /// A null-terminated C-string literal: `c"foo"`.
    CStr(Vec<u8>),

    /// A byte literal: `b'a'`.
    Byte(Vec<u8>),

    /// An int literal: `1isize`.
    Int(Vec<u8>),

    /// A float literal: `0f64`.
    Float(Vec<u8>),

    /// A character literal: `'a'`.
    Char(Vec<u8>),

    /// A boolean literal: `true` or `false`.
    Bool(Vec<u8>),

    /// A sequence literal containing any literals: `["1", "2", "3"]` or `(99, "20", [3, 4])`.
    Sequence {
        inner: Vec<Self>,
        delimiter: Delimiter,
    },
}

/// Sequence delimiters
pub enum Delimiter {
    /// Square brackets `[]` used for arrays
    Bracket,
    /// Parentheses `()` used for tuples
    Parenthesis,
}

impl LiteralBytes {
    /// Parses a token stream into a [`LiteralBytes`] enum variant
    ///
    /// # Arguments
    ///
    /// * `tokens` - Input token stream representing a Rust literal
    pub fn parse(tokens: TokenStream) -> Result<Self, &'static str> {
        let Ok(expr) = syn::parse2::<Expr>(tokens) else {
            return Err(GLOBAL_ERROR);
        };

        if let Expr::Array(ExprArray { elems, .. }) | Expr::Tuple(ExprTuple { elems, .. }) = &expr {
            let inner = elems
                .iter()
                .map(Expr::to_token_stream)
                .map(Self::parse)
                .collect::<Result<Vec<_>, _>>()?;

            return Ok(Self::Sequence {
                inner,
                delimiter: match &expr {
                    Expr::Array(_) => Delimiter::Bracket,
                    Expr::Tuple(_) => Delimiter::Parenthesis,
                    _ => unreachable!(),
                },
            });
        }

        let (lit, negative) = match &expr {
            Expr::Lit(lit) => (lit, false),
            Expr::Unary(ExprUnary {
                expr,
                op: UnOp::Neg(_),
                ..
            }) => {
                let Expr::Lit(lit) = expr.as_ref() else {
                    return Err(GLOBAL_ERROR);
                };
                if !matches!(lit.lit, Lit::Int(_) | Lit::Float(_)) {
                    return Err(GLOBAL_ERROR);
                }

                (lit, true)
            }
            _ => return Err(GLOBAL_ERROR),
        };

        let literal = match &lit.lit {
            Lit::Str(s) => Self::Str(s.value().into_bytes()),
            Lit::ByteStr(s) => Self::ByteStr(s.value()),
            Lit::CStr(s) => Self::CStr(s.value().into_bytes_with_nul()),
            Lit::Byte(b) => Self::Byte(vec![b.value()]),
            Lit::Char(c) => Self::Char((c.value() as u32).to_ne_bytes().to_vec()),
            Lit::Int(i) => {
                const ERROR: &str = "only `isize` integer literals supported";

                let int = if negative {
                    let Ok(int) = i.base10_parse::<usize>() else {
                        return Err(ERROR);
                    };

                    // cast isize::MIN to usize to remove the sign
                    if int > isize::MIN as usize {
                        return Err(ERROR);
                    }

                    // two complement
                    let signed = (!int).wrapping_add(1) as isize;

                    signed
                } else if let Ok(int) = i.base10_parse::<isize>() {
                    int
                } else {
                    return Err(ERROR);
                };

                Self::Int(int.to_ne_bytes().to_vec())
            }
            Lit::Float(f) => {
                let Ok(mut float) = f.base10_parse::<f64>() else {
                    return Err("only `f64` float literals supported");
                };

                if negative {
                    float = -float;
                }

                Self::Float(float.to_ne_bytes().to_vec())
            }
            Lit::Bool(b) => Self::Bool(vec![b.value as u8]),
            _ => return Err(GLOBAL_ERROR),
        };

        Ok(literal)
    }

    /// Gets the length of the literal's byte representation
    pub fn len(&self) -> usize {
        match self {
            Self::Str(v)
            | Self::ByteStr(v)
            | Self::CStr(v)
            | Self::Byte(v)
            | Self::Int(v)
            | Self::Float(v)
            | Self::Char(v)
            | Self::Bool(v) => v.len(),
            Self::Sequence { inner, .. } => inner.iter().map(Self::len).sum(),
        }
    }

    /// Get the byte representation of the literal
    ///
    /// # Returns
    ///
    /// Borrowed or owned byte representation
    pub fn as_bytes(&self) -> Cow<'_, [u8]> {
        match self {
            Self::Str(v)
            | Self::ByteStr(v)
            | Self::CStr(v)
            | Self::Byte(v)
            | Self::Int(v)
            | Self::Float(v)
            | Self::Char(v)
            | Self::Bool(v) => v.into(),
            Self::Sequence { inner, .. } => inner
                .iter()
                .map(Self::as_bytes)
                .flat_map(|b| b.to_vec())
                .collect::<Vec<_>>()
                .into(),
        }
    }
}

impl LiteralBytes {
    /// Encrypts itself inside a runtime decryption code.
    pub fn encrypt(self) -> TokenStream {
        let mut bytes = self.as_bytes().to_vec();
        xor(&mut bytes, &KEY);

        let decrypted = quote! {
            let mut bytes = {
                let mut bytes = [#(#bytes),*];
                crate::__boo::xor(&mut bytes, crate::BOO_KEY);
                bytes
            };
        };

        unsafe { Self::decrypt(self, decrypted, bytes.len()) }
    }

    /// Decrypts a `LiteralBytes` into its original form by interpreting the decrypted byte buffer.
    ///
    /// # Arguments
    ///
    /// * `literal` - The encrypted literal type to decrypt
    /// * `decrypted` - TokenStream containing a `bytes: [u8; N]` binding that holds the decrypted data
    /// * `decrypted_len` - Length of the `decrypted` bytes sequence
    ///
    /// # Safety
    ///
    /// Performs unchecked conversions based on the assumption that:
    ///
    /// - `decrypted` contains exactly the number of bytes expected for the target literal type.
    /// - The byte layout is valid for conversion into the target type.
    /// - `decrypted_len` match exactly the available `decrypted` bytes length.
    ///
    /// # Memory Model
    ///
    /// - String types are heap-allocated.
    /// - Other types are reinterpreted directly from the stack.
    ///
    /// # Performance
    ///
    /// Extensive usage of const operations for minimal runtime overhead.
    unsafe fn decrypt(
        literal: LiteralBytes,
        decrypted: TokenStream,
        mut decrypted_len: usize,
    ) -> TokenStream {
        match literal {
            LiteralBytes::Str(_) => quote! {
                unsafe {
                    #decrypted
                    ::alloc::string::String::from_utf8_unchecked(bytes.to_vec())
                }
            },
            LiteralBytes::ByteStr(_) => quote! {{
                #decrypted
                bytes
            }},
            LiteralBytes::CStr(_) => quote! {
                unsafe {
                    #decrypted
                    ::alloc::ffi::CString::from_vec_with_nul_unchecked(bytes.to_vec())
                }
            },
            LiteralBytes::Byte(_) => quote! {{
                #decrypted
                // Extra compile time type assertion
                (bytes as [u8; 1])[0]
            }},
            LiteralBytes::Int(_) => quote! {{
                #decrypted
                isize::from_ne_bytes(bytes)
            }},
            LiteralBytes::Float(_) => quote! {{
                #decrypted
                f64::from_ne_bytes(bytes)
            }},
            LiteralBytes::Char(_) => quote! {
                unsafe {
                    #decrypted
                    ::core::char::from_u32_unchecked(u32::from_ne_bytes(bytes))
                }
            },
            LiteralBytes::Bool(_) => quote! {{
                #decrypted
                // Extra compile time type assertion
                (bytes as [u8; 1])[0] != 0
            }},
            LiteralBytes::Sequence { inner, delimiter } => {
                let (bytes, items): (Vec<_>, Vec<_>) = inner
                    .into_iter()
                    .enumerate()
                    .map(|(i, literal)| {
                        let item_var = format!("item_{i}").parse::<TokenStream>().unwrap();
                        let item_len = literal.len();
                        let remaining_len = decrypted_len
                            .checked_sub(item_len)
                            .expect("Sequence item exceeds remaining bytes");

                        let bytes = quote! {
                            let (#item_var, bytes) = crate::__boo::split_array::<
                                #decrypted_len,
                                #item_len,
                                #remaining_len
                            >(bytes);
                        };

                        let item_bytes = quote! { let mut bytes = #item_var; };
                        let decrypted_item =
                            unsafe { Self::decrypt(literal, item_bytes, item_len) };

                        // Update remaining length for next iteration
                        decrypted_len = remaining_len;

                        (bytes, decrypted_item)
                    })
                    .unzip();

                let sequence = match delimiter {
                    Delimiter::Bracket => quote! { [#(#items),*] },
                    Delimiter::Parenthesis => quote! { (#(#items),*) },
                };

                quote! {{
                    #decrypted
                    #(#bytes)*
                    #sequence
                }}
            }
        }
    }
}
