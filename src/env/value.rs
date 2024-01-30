use super::Secret;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum Value {
    Value(String),
    Secret(Secret),
}

impl Value {
    pub(crate) fn to_revel(&self) -> &str {
        match self {
            Value::Value(value) => value,
            Value::Secret(secret) => secret.to_revel(),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Value(value) => f.write_str(value),
            Value::Secret(secret) => f.write_str(&secret.to_string()),
        }
    }
}
