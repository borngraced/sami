use std::env;

use super::error::{SamiError, SamiErrorWithData};

pub fn db_from_env(name: &str) -> Result<String, SamiError> {
    let env = env::var(name).map_err(|e| {
        SamiError::InvalidError(SamiErrorWithData {
            field: "db_env".to_string(),
            message: e.to_string(),
        })
    })?;

    let res = env.replace("/", " ").replace("-", "=");
    Ok(res)
}
