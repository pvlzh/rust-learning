use crate::constants::PW_DEFAULT_LENGTH;

/// Password generation settings.
#[derive(Debug)]
pub struct PasswordOptions{
    /// Password length.
    pub length: u8,
    /// Should the password contain numbers.
    pub numbers: bool,
    /// Should the password contain symbols.
    pub symbols: bool,
    /// Should the password contain uppercase characters.
    pub uppercase: bool 
}

/// Implementing "Default" trait for PasswordOptions.
impl Default for PasswordOptions {
    fn default() -> Self {
        Self { 
            length: PW_DEFAULT_LENGTH, 
            numbers: false,
            symbols: false,
            uppercase: false
        }
    }
}