//! Stub module for prettier crate
//! The prettier crate was removed in Phase 3. This stub provides minimal types for compilation.

use anyhow::Result;
use gpui::{App, Model};

#[derive(Clone)]
pub struct Prettier;

impl Prettier {
    pub fn new() -> Self {
        Prettier
    }

    pub fn global(_cx: &App) -> Option<Model<Self>> {
        // Prettier not supported in terminal fork
        None
    }
}

#[derive(Clone, Debug)]
pub struct PrettierSettings {
    pub enabled: bool,
}

impl Default for PrettierSettings {
    fn default() -> Self {
        Self { enabled: false }
    }
}

pub fn init() -> Result<()> {
    // Prettier not needed in terminal fork
    Ok(())
}
