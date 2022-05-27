use std::env;

use super::error::{ErrorResponse, SamiStatusCode};

pub fn data_from_env(name: &str) -> Result<String, ErrorResponse> {
    let env = env::var(name).map_err(|e| ErrorResponse {
        field: None,
        message: Some(e.to_string()),
        code: SamiStatusCode::Sql,
    })?;

    match name.to_lowercase().as_str() {
        "db" => {
            let res = env.replace("/", " ").replace("*", "=");
            Ok(res)
        }
        _ => Ok(env),
    }
}
