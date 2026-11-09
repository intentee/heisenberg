use cargo::context::ConfigKey;

#[must_use]
pub fn cargo_environment_key(parts: &[&str]) -> String {
    let mut key = ConfigKey::new();

    for part in parts {
        key.push(part);
    }

    key.as_env_key().to_owned()
}
