//! herdr-runtime — server runtime for herdr
//!
//! Contains the event loop, PTY lifecycle, session restore, workspace/tab/pane
//! management, and all server-side state mutations.

pub mod app;
pub mod agent_resume;
pub mod config_io;
pub mod creation;
pub mod loop_event;
pub mod popup;
pub mod runtime;
pub mod runtime_mutations;
pub mod session;
pub mod state_ext;
pub mod terminal_targets;
pub mod terminal_titles;
pub mod theme_sync;
pub mod worktrees;
pub mod agents;