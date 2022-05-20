use regex::Regex;
use scrypt::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Scrypt,
};

use super::error::SamiError;

pub fn validate_email(email: &String) -> Result<String, SamiError> {
    debug!("Validation email {}../", &email);

    let email_regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();
    if let false = email_regex.is_match(email.as_str()) {
        return Err(SamiError::InvalidEmail {
            field: email.to_string(),
        });
    }
    Ok(email.to_string())
}

pub fn encode_password(password: String) -> Result<String, String> {
    debug!("Encoding password");

    let password = password.as_bytes(); // Bad password; don't actually use!
    let salt = SaltString::generate(&mut OsRng);

    // Hash password to PHC string ($scrypt$...)
    let password_hash = Scrypt.hash_password(password, &salt);
    match password_hash {
        Ok(e) => Ok(e.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

pub fn verify_password(password: String, password_hash: String) -> Result<String, String> {
    debug!("Verifying password");

    let parsed_hash = PasswordHash::new(&password_hash).expect("password not hashable");
    let verify = Scrypt
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();
    match verify {
        true => Ok(password),
        false => Err("Invalid Password".to_string()),
    }
}

// pub fn validate_password(pass_word: &String) -> Result<String, Error> {
//     let username = Regex::new(r"[a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?").expect("regex error");
//     let username = username.is_match(pass_word.as_str());
//     if let false = username {
//         println!("error occured");
//     }
//     Ok(pass_word.to_string())
// }

#[cfg(test)]
mod tests {
    use super::{encode_password, validate_email, verify_password};

    #[test]
    fn en_n_de_code_ps() {
        let ecode = encode_password("SamopE160!".to_string()).unwrap();
        let dcode = verify_password("SamopE160!".to_string(), ecode.clone()).unwrap();
        assert_eq!("SamopE160!".to_string(), dcode);
    }

    #[test]
    fn validate_email_test() {
        assert_eq!(
            "sami@gmail.com".to_string(),
            validate_email(&"sami@gmail.com".to_string()).unwrap()
        )
    }
}
