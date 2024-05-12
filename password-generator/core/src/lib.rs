pub mod options;
mod constants;

use rand::Rng;

use crate::{constants::*, options::PasswordOptions};

/// Password generation.
pub fn generate_password(opt: PasswordOptions) -> Result<String, &'static str> {
    if opt.length < PW_MIN_LENGTH {
        return Err("Длина генерируемого пароля не может быть меньше 4 символов");
    }
    else if opt.length > PW_MAX_LENGTH {
        return Err("Максимальная длина генерируемого пароля - 128 символов")
    }

    let mut character_pool: Vec<&[char]> = Vec::new();
    character_pool.push(&LOWERCASE_CHARACTERS);

    if opt.numbers {
        character_pool.push(&NUMBERS)
    }
    if opt.symbols {
        character_pool.push(&SYMBOLS);
    }
    if opt.uppercase {
        character_pool.push(&UPPERCASE_CHARACTERS)
    }

    let mut password = String::new();
    for ind in 0..opt.length as usize {
        let rand_charset = rand::thread_rng()
            .gen_range(0..character_pool.len());

        let rand_char = get_rand_char(character_pool[rand_charset]);
        password.insert(ind, rand_char);
    }

    loop {
        if !has_character(&password, &LOWERCASE_CHARACTERS) {
            rand_insert(&mut password, &LOWERCASE_CHARACTERS);
            continue;
        }
        else if opt.numbers && !has_character(&password, &NUMBERS) {
            rand_insert(&mut password, &NUMBERS);
            continue;
        }
        else if opt.symbols && !has_character(&password, &SYMBOLS) {
            rand_insert(&mut password, &SYMBOLS);
            continue;
        }
        else if opt.uppercase && !has_character(&password, &UPPERCASE_CHARACTERS) {
            rand_insert(&mut password, &UPPERCASE_CHARACTERS);
            continue;
        }
        break;
    }

    return Ok(password);
}

/// Take a random character from the sequence of characters.
fn get_rand_char(charset: &[char]) -> char {
    let r_ch_id = rand::thread_rng()
        .gen_range(0..charset.len());
    charset[r_ch_id]
}

/// Insert a random character of the sequence 
/// into a random location in the string.
fn rand_insert(str_value: &mut String, charset: &[char]){
    let rand_char = get_rand_char(charset);
    let rand_str_inx = rand::thread_rng()
        .gen_range(0..str_value.len());
    str_value.remove(rand_str_inx);
    str_value.insert(rand_str_inx, rand_char);
}

/// Checking that the string contains at least one character 
/// of the sequence of characters.
fn has_character(str_value: &String, charset: &[char]) -> bool{
    str_value.chars().any(|ch| { charset.contains(&ch)})
}



#[cfg(test)]
mod tests {
    use crate::constants::PW_DEFAULT_LENGTH;
    use super::*;

    /// Generation test with "Default" settings.
    #[test]
    fn validate_default_pw() {
        // Arrange
        let options = PasswordOptions::default();
        // Act
        let password = generate_password(options);
        // Assert
        assert!(password.is_ok());
        assert_eq!(PW_DEFAULT_LENGTH, password.unwrap().len() as u8);
    }
    
    /// Test the length of the generated password.
    #[test]
    fn validate_pw_length() {
        // Arrange
        const EXPECTED_LEN: u8 = 38;
        let options = PasswordOptions{ 
            length: EXPECTED_LEN,
            numbers: false,
            symbols: false,
            uppercase: false };
        // Act
        let password = generate_password(options);
        // Assert
        assert!(password.is_ok());
        let password = password.unwrap();
        assert_eq!(EXPECTED_LEN, password.len() as u8);
        assert!(has_character(&password, &LOWERCASE_CHARACTERS));
    }
    
    /// 
    #[test]
    fn validate_pw_with_numbers() {
        // Arrange
        let options = PasswordOptions{ 
            length: PW_DEFAULT_LENGTH,
            numbers: true,
            symbols: false,
            uppercase: false };
        // Act
        let password = generate_password(options);
        // Assert
        assert!(password.is_ok());
        let password = password.unwrap();
        assert!(has_character(&password, &LOWERCASE_CHARACTERS));
        assert!(has_character(&password, &NUMBERS));
    }

    /// 
    #[test]
    fn validate_pw_with_symbols() {
        // Arrange
        let options = PasswordOptions{ 
            length: PW_DEFAULT_LENGTH,
            numbers: false,
            symbols: true,
            uppercase: false };
        // Act
        let password = generate_password(options);
        // Assert
        assert!(password.is_ok());
        let password = password.unwrap();
        assert!(has_character(&password, &LOWERCASE_CHARACTERS));
        assert!(has_character(&password, &SYMBOLS));
    }

    /// 
    #[test]
    fn validate_pw_with_uppercase() {
        // Arrange
        let options = PasswordOptions{ 
            length: PW_DEFAULT_LENGTH,
            numbers: false,
            symbols: false,
            uppercase: true };
        // Act
        let password = generate_password(options);
        // Assert
        assert!(password.is_ok());
        let password = password.unwrap();
        assert!(has_character(&password, &LOWERCASE_CHARACTERS));
        assert!(has_character(&password, &UPPERCASE_CHARACTERS));
    }

    /// has_character method test.
    #[test]
    fn has_character_test(){
        let value = String::from("aBcDe");
        assert!(has_character(&value, &LOWERCASE_CHARACTERS));
        assert!(has_character(&value, &UPPERCASE_CHARACTERS));
        assert!(!has_character(&value, &NUMBERS));
        assert!(!has_character(&value, &SYMBOLS));

        let value = String::from("45%*12");
        assert!(has_character(&value, &NUMBERS));
        assert!(has_character(&value, &SYMBOLS));
        assert!(!has_character(&value, &LOWERCASE_CHARACTERS));
        assert!(!has_character(&value, &UPPERCASE_CHARACTERS));
    }
}