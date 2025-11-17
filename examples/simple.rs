//! Boo Usage Examples
//!
//! Demonstrates secure string handling with compile-time encryption
//! and runtime decryption.

extern crate alloc;
#[macro_use]
extern crate boo;

// Initialize encryption with project-specific keys
boo_init!();

/// Wraps an encrypted string in a function. Equivalent to:
///
/// ```
/// static SECRET: &str = "secret-api-key";
/// ```
///
/// but with runtime decryption per access.
///
/// # Example
///
/// ```
/// let key = api_key();  // Decrypted in memory
/// authenticate(&key);   // Used while decrypted
/// drop(key);            // Memory is secured after dropping
/// ```
///
/// # Note
///
/// Each call decrypts the string, no persistent decrypted state.
fn api_key() -> String {
    boo!("secret-api-key")
}

fn authenticate(_: &str) {}

fn main() {
    // Example 1: Scoped usage
    {
        let key = api_key(); // Decrypted here
        authenticate(&key);
    } // Secured here

    // Example 2: Immediate usage
    // "secret-api-key" decrypted and secured within method call
    authenticate(&boo!("secret-api-key"));
    // This also works, the string will not remain in memory
    println!("Api key: {}", boo!("secret-api-key"));

    // Example 3: Secure comparison
    // "password" decrypted only for comparison
    if "password" == boo!("password") {
        println!("Access granted");
    }

    // Example 4: Unsafe pattern (avoid)
    let secret = boo!("top-secret"); // Remains decrypted until drop
    // ... rest of the program ...
    drop(secret); // Explicit cleanup recommended
}
