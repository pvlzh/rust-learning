/// Standard length of generated password.
pub const PW_DEFAULT_LENGTH: u8 = 8;
/// Minimum length of generated password.
pub const PW_MIN_LENGTH: u8 = 4;
/// Maximum length of generated password.
pub const PW_MAX_LENGTH: u8 = 128;

/// Enumeration of numeric characters.
pub const NUMBERS: [char; 8] = [
    '2', '3', '4', '5', '6', '7', '8', '9'];
/// Enumeration of numeric characters.
pub const LOWERCASE_CHARACTERS: [char; 23] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 
    'j', 'k', 'm', 'n', 'p', 'q', 'r', 's', 
    't', 'u', 'v','w', 'x', 'y', 'z'];
/// Enumeration of lowercase alphabetic characters.
pub const UPPERCASE_CHARACTERS: [char; 24] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 
    'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 
    'S', 'T', 'U','V', 'W', 'X', 'Y', 'Z',];
/// Enumeration of symbol characters.
pub const SYMBOLS: [char; 28] = [
    '!', '#', '$', '%', '&', '(', ')', '*', 
    '+', ',', '-', '.', '/', ':', ';', '<', 
    '=', '>', '?','@', '[', '\\', ']', '^', 
    '_', '{', '}', '~',];