// This is a temporary function for validating the entered password.
// Rust has a native CSV reader function. We should be using that to check passwords.
pub fn check_passkey(passkey_encrypted: &str) -> bool {
    let password_list = vec!["Password", "Wordpass"];

    password_list.contains(&passkey_encrypted)
}