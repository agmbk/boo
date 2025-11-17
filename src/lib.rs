//! # Boo

extern crate alloc;
extern crate core;
extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate rand;
extern crate syn;

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use proc_macro2::Literal;
use quote::quote;
use syn::{Expr, ExprLit, Lit};

use crate::literal_bytes::LiteralBytes;

mod literal_bytes;
#[cfg(test)]
mod test;
mod utils;

const INCLUDE_ERROR: &str = r#"expected one file path (ex. "data.txt")"#;

/// Cryptographic key
static KEY: LazyLock<Box<[u8]>> = LazyLock::new(|| match option_env!("BOO_KEY") {
    Some(key) => key.as_bytes().into(),
    None => {
        let mut key = [0; 64];
        rand::fill(&mut key);

        key.into()
    }
});

/// Initialize the boo library allowing use of the boo macros.
///
/// Optionally set a custom key using the `BOO_KEY` environment variable.
/// Fallbacks to a random 64-bytes cryptographic key.
#[proc_macro]
pub fn boo_init(_tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let key = Literal::byte_string(&KEY);
    let utils = syn::parse_str::<syn::File>(include_str!("utils.rs")).unwrap();

    let result = quote! {
        static BOO_KEY: &[u8] = #key;

        pub mod __boo {
            #utils
        }
    };

    result.into()
}

/// Encrypts a literal
#[proc_macro]
pub fn boo(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let literal = match LiteralBytes::parse(tokens.into()) {
        Ok(literal) => literal,
        Err(err) => panic!("{err}"),
    };

    literal.encrypt().into()
}

/// Encrypts a raw file as bytes
#[proc_macro]
pub fn boo_include_bytes(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Some(file_path) = read_literal_str(tokens) else {
        panic!("{INCLUDE_ERROR}");
    };
    let file_path = relative_path(&file_path);

    let data = match fs::read(file_path) {
        Ok(data) => data,
        Err(err) => panic!("Failed to read the file: {err}"),
    };

    LiteralBytes::ByteStr(data).encrypt().into()
}

/// Encrypts a UTF-8 file as a string
#[proc_macro]
pub fn boo_include_str(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Some(file_path) = read_literal_str(tokens) else {
        panic!("{INCLUDE_ERROR}");
    };
    let file_path = relative_path(&file_path);

    let data = match fs::read_to_string(file_path) {
        Ok(data) => data,
        Err(err) => panic!("Failed to read the file: {err}"),
    };

    LiteralBytes::Str(data.into_bytes()).encrypt().into()
}

/// Reads a single string literal from a token stream
///
/// # Arguments
///
/// * `tokens` - Token stream containing a single string literal
fn read_literal_str(tokens: proc_macro::TokenStream) -> Option<String> {
    if let Ok(expr) = syn::parse2::<Expr>(tokens.into()) {
        match expr {
            Expr::Lit(ExprLit {
                lit: Lit::Str(str), ..
            }) => return Some(str.value()),
            _ => {}
        }
    };

    None
}

/// Makes a path relative to the calling source code file
fn relative_path(path: &str) -> PathBuf {
    let current_dir = Path::new(file!())
        .parent()
        .unwrap_or_else(|| Path::new("."));

    current_dir.join(path)
}
