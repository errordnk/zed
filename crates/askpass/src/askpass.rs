//! Stub module for askpass crate
//! The askpass crate was removed in Phase 2. This stub provides minimal types for compilation.

use anyhow::Result;
use futures::channel::oneshot;
use gpui::AsyncApp;

#[derive(Clone)]
pub struct AskPassDelegate;

impl AskPassDelegate {
    pub fn new<F>(_cx: &AsyncApp, _callback: F) -> Self
    where
        F: Fn(String, oneshot::Sender<Result<String>>, AsyncApp) + 'static + Send + Sync,
    {
        Self
    }

    pub fn ask_password(&self, _prompt: String) -> impl std::future::Future<Output = Result<String>> {
        async { Ok(String::new()) }
    }
}

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
