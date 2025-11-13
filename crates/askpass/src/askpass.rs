//! Stub module for askpass crate
//! The askpass crate was removed in Phase 2. This stub provides minimal types for compilation.

use anyhow::Result;

#[derive(Clone)]
pub struct AskPassDelegate;

pub struct EncryptedPassword(String);

pub struct IKnowWhatIAmDoingAndIHaveReadTheDocs;

impl EncryptedPassword {
    pub fn decrypt(&self, _marker: IKnowWhatIAmDoingAndIHaveReadTheDocs) -> Result<String> {
        Ok(self.0.clone())
    }
}

impl TryFrom<&str> for EncryptedPassword {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        Ok(EncryptedPassword(value.to_string()))
    }
}
