use chrono::Utc;
use pasetors::claims::{Claims, ClaimsValidationRules};
use pasetors::keys::SymmetricKey;
use pasetors::local;
use pasetors::token::UntrustedToken;
use pasetors::version4::V4;
use pasetors::Local;
use serde_json::Value;

lazy_static::lazy_static! {
    static ref PASETO_KEY: SymmetricKey<V4> = {
        let secret = crate::config::refresh_key();
        SymmetricKey::<V4>::from(secret.as_bytes()).expect("密钥必须是32字节")
    };
}

pub fn create_token(username: &str) -> String {
    let mut claims = Claims::new().unwrap();
    claims.add_additional("username", username).unwrap();

    let expiration = (Utc::now() + chrono::Duration::hours(24)).to_rfc3339();
    claims.expiration(&expiration).unwrap();

    local::encrypt(&PASETO_KEY, &claims, None, None).unwrap()
}

pub fn verify_token(token: &str) -> Result<String, String> {
    let untrusted_token =
        UntrustedToken::<Local, V4>::try_from(token).map_err(|_| "令牌格式无效".to_string())?;

    let validation_rules = ClaimsValidationRules::new();

    match local::decrypt(&PASETO_KEY, &untrusted_token, &validation_rules, None, None) {
        Ok(trusted_token) => {
            let payload = trusted_token.payload();
            let claims: Value = serde_json::from_str(payload).map_err(|_| "解析声明失败")?;
            let username = claims
                .get("username")
                .and_then(|v| v.as_str())
                .ok_or("用户声明缺失")?;
            Ok(username.to_string())
        }
        Err(_) => Err("令牌解密失败".to_string()),
    }
}
