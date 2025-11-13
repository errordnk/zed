//! Stub module for prettier crate
//! The prettier crate was removed in Phase 3. This stub provides minimal types for compilation.

use anyhow::Result;
use gpui::{App, AsyncApp, Entity, Task};
use language::Buffer;
use std::path::{Path, PathBuf};
use std::sync::Arc;

// Constants
pub const FAIL_THRESHOLD: usize = 10;
pub const PRETTIER_SERVER_FILE: &str = "prettier_server.js";
pub const PRETTIER_SERVER_JS: &str = include_str!("../prettier_server.js");

#[derive(Clone)]
pub struct Prettier;

#[derive(Clone)]
pub struct PrettierServer;

impl Prettier {
    pub fn new() -> Self {
        Prettier
    }

    pub fn global(_cx: &App) -> Option<Entity<Self>> {
        // Prettier not supported in terminal fork
        None
    }

    pub fn locate_prettier_installation(
        _worktree_paths: impl Iterator<Item = Arc<Path>>,
        _locate_in: Vec<PathBuf>,
        _node: node_runtime::NodeRuntime,
        _cx: &mut AsyncApp,
    ) -> Task<Result<Option<PathBuf>>> {
        Task::ready(Ok(None))
    }

    pub fn locate_prettier_ignore(
        _fs: Arc<dyn fs::Fs>,
        _worktree_paths: impl Iterator<Item = Arc<Path>>,
    ) -> Task<Result<Vec<PathBuf>>> {
        Task::ready(Ok(Vec::new()))
    }

    pub fn format(
        &self,
        _buffer: Entity<Buffer>,
        _buffer_path: Option<PathBuf>,
        _ignore_dir: Option<PathBuf>,
        _cx: &mut AsyncApp,
    ) -> Task<Result<language::Diff>> {
        Task::ready(Err(anyhow::anyhow!("Prettier not available in terminal fork")))
    }

    pub fn server(&self) -> Option<Arc<PrettierServer>> {
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
