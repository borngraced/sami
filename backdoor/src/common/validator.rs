use regex::Regex;
use scrypt::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Scrypt,
};
use std::{
    fs::File,
    io::{BufReader, Read},
};

use crate::common::error::{SamiErrorWithData, SamiStatusCode};

use super::error::SamiError;

pub fn validate_username(username: &str) -> Result<String, SamiError> {
    debug!("Validating username {}../", &username);

    let username_regex = Regex::new(r"[a-zA-Z]{2,}\d*$/i").unwrap();

    if username_regex.is_match(username) || username.contains(" ") || username.len() < 5 {
        return Err(SamiError::BadRequest(SamiErrorWithData {
            field: "username".to_string(),
            message: "Please use a valid username".to_string(),
        }));
    }
    Ok(username.to_string())
}

pub fn validate_email(email: &String) -> Result<String, SamiError> {
    debug!("Validation email {}../", &email);

    let email_regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();
    if let false = email_regex.is_match(email.as_str()) {
        return Err(SamiError::BadRequest(SamiErrorWithData {
            field: "email".to_string(),
            message: "Invalid email address".to_string(),
        }));
    }
    Ok(email.to_string())
}

pub fn validate_password(password: &str) -> Result<String, SamiError> {
    debug!("Validation password {}../", &password);

    if password.len() < 6 || password.contains(" ") {
        return Err(SamiError::BadRequest(SamiErrorWithData {
            field: "password".to_string(),
            message: "Password too short! must be atleast 6 chars and no space allowed".to_string(),
        }));
    }

    let mut common_password = File::open("./src/common_passwords.txt").map_err(|_e| {
        SamiError::BadRequest(SamiErrorWithData {
            field: "password".to_string(),
            message: "Unknown error while validating your password".to_string(),
        })
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
        return Err(SamiError::BadRequest(SamiErrorWithData {
            field: "password".to_string(),
            message: "Password can be easily guessed please choose a stronger password".to_string(),
        }));
    };

    Ok(password.to_string())
}

pub fn encode_password(password: String) -> Result<String, SamiError> {
    debug!("Encoding password");

    let password = password.as_bytes(); // Bad password; don't actually use!
    let salt = SaltString::generate(&mut OsRng);

    // Hash password to PHC string ($scrypt$...)
    let password_hash = Scrypt.hash_password(password, &salt);
    match password_hash {
        Ok(e) => Ok(e.to_string()),
        Err(e) => Err(SamiError::BadRequest(SamiErrorWithData {
            field: "password".to_string(),
            message: e.to_string(),
        })),
    }
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<(), SamiError> {
    debug!("Verifying password");

    let parsed_hash = PasswordHash::new(&password_hash).expect("password not hashable");
    let verify = Scrypt
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();
    match verify {
        true => Ok(()),
        false => Err(SamiError::BadRequest(SamiErrorWithData {
            field: "password".to_string(),
            message: "Password Incorrect".to_string(),
        })),
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use crate::common::error::{SamiError, SamiErrorWithData};

    use super::{
        encode_password, validate_email, validate_password, validate_username, verify_password,
    };

    #[test]
    fn en_n_de_code_ps() {
        let ecode = encode_password("SamopE160!".to_string()).unwrap();
        let dcode = verify_password("SamopE160!", &ecode).unwrap();
        assert_eq!((), dcode);
    }

    #[test]
    fn validate_email_test() {
        assert_eq!(
            "sami@gmail.com".to_string(),
            validate_email(&"sami@gmail.com".to_string()).unwrap()
        )
    }

    #[test]
    fn validate_username_test() {
        assert_eq!(
            "bsto0403".to_string(),
            validate_username(&"bsto0403".to_string()).unwrap()
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
        let err1 = SamiError::BadRequest(SamiErrorWithData {
            field: "password".to_string(),
            message: "Password too short! must be atleast 6 chars and no space allowed".to_string(),
        });
        assert_eq!(err1, validate_password("gjjg").err().unwrap());

        // test common password error
        let err2 = SamiError::BadRequest(SamiErrorWithData {
            field: "password".to_string(),
            message: "Password can be easily guessed please choose a stronger password".to_string(),
        });
        assert_eq!(err2, validate_password("playstation").err().unwrap())
    }
}
