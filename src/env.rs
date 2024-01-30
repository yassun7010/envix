mod secret;
mod value;

pub use secret::Secret;
pub use value::Value;

pub trait ExpandEnvs {
    fn expand_envs(
        &self,
        stage: Option<&str>,
    ) -> Result<indexmap::IndexMap<&str, Value>, crate::Error>;
}
