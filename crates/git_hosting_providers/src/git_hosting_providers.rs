//! Stub module for git_hosting_providers crate
//! The git_hosting_providers crate was removed in Phase 3. This stub provides minimal types for compilation.

use anyhow::Result;
use url::Url;

#[derive(Clone, Debug)]
pub struct GitHostingProvider {
    pub name: String,
    pub url: Option<Url>,
}

impl GitHostingProvider {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            url: None,
        }
    }

    pub fn github() -> Self {
        Self::new("GitHub")
    }

    pub fn gitlab() -> Self {
        Self::new("GitLab")
    }

    pub fn bitbucket() -> Self {
        Self::new("Bitbucket")
    }
}

#[derive(Clone, Debug)]
pub struct ParsedGitRemote {
    pub provider: Option<GitHostingProvider>,
    pub owner: String,
    pub repo: String,
}

pub fn parse_git_remote_url(_url: &str) -> Option<ParsedGitRemote> {
    // Git hosting providers integration not needed in terminal fork
    None
}

pub fn register_additional_providers(_providers: Vec<GitHostingProvider>) {
    // Git hosting providers registration not needed in terminal fork
}

pub fn init() -> Result<()> {
    // Git hosting providers not needed in terminal fork
    Ok(())
}
