use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use anyhow::Result;
use credentials_provider::CredentialsProvider;
use futures::FutureExt as _;
use gpui::{App, AsyncApp, Global};

pub struct ZedCredentialsProvider(pub Arc<dyn CredentialsProvider>);

impl Global for ZedCredentialsProvider {}

/// Returns the global [`CredentialsProvider`].
pub fn init_global(cx: &mut App) {
    // The `CredentialsProvider` trait has `Send + Sync` bounds on it, so it
    // seems like this is a false positive from Clippy.
    #[allow(clippy::arc_with_non_send_sync)]
    let provider = new(cx);
    cx.set_global(ZedCredentialsProvider(provider));
}

pub fn global(cx: &App) -> Arc<dyn CredentialsProvider> {
    cx.try_global::<ZedCredentialsProvider>()
        .map(|provider| provider.0.clone())
        .unwrap_or_else(|| new(cx))
}

fn new(_cx: &App) -> Arc<dyn CredentialsProvider> {
    Arc::new(KeychainCredentialsProvider)
}

/// A credentials provider that stores credentials in the system keychain.
struct KeychainCredentialsProvider;

impl CredentialsProvider for KeychainCredentialsProvider {
    fn read_credentials<'a>(
        &'a self,
        url: &'a str,
        cx: &'a AsyncApp,
    ) -> Pin<Box<dyn Future<Output = Result<Option<(String, Vec<u8>)>>> + 'a>> {
        async move { cx.update(|cx| cx.read_credentials(url)).await }.boxed_local()
    }

    fn write_credentials<'a>(
        &'a self,
        url: &'a str,
        username: &'a str,
        password: &'a [u8],
        cx: &'a AsyncApp,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>> {
        async move {
            cx.update(move |cx| cx.write_credentials(url, username, password))
                .await
        }
        .boxed_local()
    }

    fn delete_credentials<'a>(
        &'a self,
        url: &'a str,
        cx: &'a AsyncApp,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>> {
        async move { cx.update(move |cx| cx.delete_credentials(url)).await }.boxed_local()
    }
}

