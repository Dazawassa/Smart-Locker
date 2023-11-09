// This should be private but we just need this to work right now.
pub fn encrypt_text(passkey: &str) -> String {
    // Perform encryption logic here.
    // For example, let's just return the passkey as is for demonstration purposes.
    let passkey_encrypted = passkey.to_string();
    // Value we pass into main.
    passkey_encrypted
}