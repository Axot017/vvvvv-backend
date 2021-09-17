use std::env;

pub struct ProfileConfig {
    pub email_verification_key_exp: u64,
}

impl ProfileConfig {
    pub fn new() -> ProfileConfig {
        let email_verification_key_exp = env::var("EMAIL_VERIFICATION_KEU_EXP")
            .unwrap_or("".to_string())
            .parse::<u64>()
            .unwrap_or(10_800);
        return ProfileConfig {
            email_verification_key_exp,
        };
    }
}
