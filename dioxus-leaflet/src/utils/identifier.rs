#[derive(Clone)]
pub(crate) struct IdentifiedElementSpecs(String);

impl IdentifiedElementSpecs {
    pub fn new<T: AsRef<str>>(id: T) -> Self {
        Self(id.as_ref().to_string())
    }

    pub fn id(&self) -> &str {
        &self.0
    }
}
