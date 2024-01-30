#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct Secret(String);

const SECRET_MASK: &str = "********************";

impl Secret {
    pub(crate) fn to_revel(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(SECRET_MASK)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret() {
        let secret = Secret("secret".to_owned());
        assert_eq!(secret.to_string(), SECRET_MASK);
    }

    #[test]
    fn test_secret_from_json() -> crate::tests::Result {
        let secret: Secret = serde_json::from_str("\"secret\"")?;
        assert_eq!(secret.to_string(), SECRET_MASK);

        Ok(())
    }
}
