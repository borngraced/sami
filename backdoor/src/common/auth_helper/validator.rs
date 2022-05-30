use regex::Regex;
use scrypt::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Scrypt,
};
use std::{
    fs::File,
    io::{BufReader, Read},
};

use crate::common::error::{ErrorResponse, SamiStatusCode};

pub fn validate_username(username: &str) -> Result<String, ErrorResponse> {
    debug!("Validating username {}../", &username);

    let username_regex = Regex::new(r"[a-zA-Z]{2,}\d*$/i").unwrap();

    if username_regex.is_match(username) || username.contains(" ") || username.len() < 5 {
        return Err(ErrorResponse {
            field: Some("username".to_string()),
            message: Some("Please use a valid username".to_string()),
            code: SamiStatusCode::Bad,
        });
    }
    Ok(username.to_string())
}

pub fn validate_email(email: &String) -> Result<String, ErrorResponse> {
    debug!("Validation email {}../", &email);

    let email_regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();
    if let false = email_regex.is_match(email.as_str()) {
        return Err(ErrorResponse {
            field: Some("email".to_string()),
            message: Some("Invalid email address".to_string()),
            code: SamiStatusCode::Bad,
        });
    }
    Ok(email.to_string())
}

pub fn validate_password(password: &str) -> Result<String, ErrorResponse> {
    debug!("Validation password {}../", &password);

    if password.len() < 6 || password.contains(" ") {
        return Err(ErrorResponse {
            field: Some("password".to_string()),
            message: Some(
                "Password too short! must be atleast 6 chars and no space allowed".to_string(),
            ),
            code: SamiStatusCode::Bad,
        });
    }

    let mut common_password =
        File::open("./src/common_passwords.txt").map_err(|_e| ErrorResponse {
            field: Some("password".to_string()),
            message: Some("Unknown error while validating your password".to_string()),
            code: SamiStatusCode::Bad,
        })?;
    let mut common_password_buffer = BufReader::new(&mut common_password);
    let mut common_password_res = String::new();
    common_password_buffer
        .read_to_string(&mut common_password_res)
        .unwrap();

    if let true = common_password_res
        .lines()
        .find(|&e| e == password)
        .is_some()
    {
        return Err(ErrorResponse {
            field: Some("password".to_string()),
            message: Some(
                "Password can be easily guessed please choose a stronger password".to_string(),
            ),
            code: SamiStatusCode::Bad,
        });
    };

    Ok(password.to_string())
}

pub fn encode_password(password: String) -> Result<String, ErrorResponse> {
    debug!("Encoding password");

    let password = password.as_bytes(); // Bad password; don't actually use!
    let salt = SaltString::generate(&mut OsRng);

    // Hash password to PHC string ($scrypt$...)
    let password_hash = Scrypt.hash_password(password, &salt);
    match password_hash {
        Ok(e) => Ok(e.to_string()),
        Err(e) => Err(ErrorResponse {
            field: Some("password".to_string()),
            message: Some(e.to_string()),
            code: SamiStatusCode::Bad,
        }),
    }
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<(), ErrorResponse> {
    debug!("Verifying password");

    let parsed_hash = PasswordHash::new(&password_hash).expect("password not hashable");
    let verify = Scrypt
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();
    match verify {
        true => Ok(()),
        false => Err(ErrorResponse {
            field: Some("password".to_string()),
            message: Some("Incorrect Password".to_string()),
            code: SamiStatusCode::Bad,
        }),
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use crate::common::error::{ErrorResponse, SamiStatusCode};

    use super::{
        encode_password, validate_email, validate_password, validate_username, verify_password,
    };

    #[test]
    fn en_n_de_code_ps() {
        let ecode = encode_password("OogeelyDoogely!".to_string()).unwrap();
        let dcode = verify_password("OogeelyDoogely!", &ecode).unwrap();
        assert_eq!((), dcode);
    }

    #[test]
    fn validate_email_test() {
        assert_eq!(
            "OogeelyDoogely@gmail.com".to_string(),
            validate_email(&"OogeelyDoogely@gmail.com".to_string()).unwrap()
        )
    }

    #[test]
    fn validate_username_test() {
        assert_eq!(
            "OogeelyDoogely".to_string(),
            validate_username(&"OogeelyDoogely".to_string()).unwrap()
        )
    }

    #[test]
    fn validate_password_test() {
        // test correct password response
        assert_eq!(
            "Gjjrkkre".to_string(),
            validate_password("Gjjrkkre").unwrap()
        );

        // test short password error
        let err1 = ErrorResponse {
            field: Some("password".to_string()),
            message: Some(
                "Password too short! must be atleast 6 chars and no space allowed".to_string(),
            ),
            code: SamiStatusCode::Bad,
        };
        assert_eq!(err1, validate_password("gjjg").err().unwrap());

        // test common password error
        let err2 = ErrorResponse {
            field: Some("password".to_string()),
            message: Some(
                "Password can be easily guessed please choose a stronger password".to_string(),
            ),
            code: SamiStatusCode::Bad,
        };
        assert_eq!(err2, validate_password("playstation").err().unwrap())
    }
}
