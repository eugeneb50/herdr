//! herdr-api — JSON API and wire protocol handlers
//!
//! Pure request handlers that take explicit dependencies instead of `&mut App`.

pub mod agents;
pub mod env;
pub mod handlers;
pub mod helpers;
pub mod internal;
pub mod integrations;
pub mod layouts;
pub mod pane_graphics;
pub mod panes;
pub mod plugins;
pub mod responses;
pub mod server;
pub mod session;
pub mod subscriptions;
pub mod tabs;
pub mod wait;
pub mod workspaces;
pub mod worktrees;

//pub use crate::server::{ApiRequestMessage, EventHub, request_changes_ui};