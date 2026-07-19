//! Timing and deadline state extracted from AppState
//!
//! This separates runtime bookkeeping from pure data in AppState.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool};
use std::sync::Arc;
use std::time::Instant;

use crate::types::{GitStatusCacheEntry, PaneId};

/// All timing-related state moved out of AppState.
/// This is the "impure" part that changes frequently and is only relevant at runtime.
#[derive(Debug)]
pub struct AppTiming {
    pub config_diagnostic_deadline: Option<Instant>,
    pub toast_deadline: Option<Instant>,
    pub copy_feedback_deadline: Option<Instant>,
    pub last_api_notification_at: Option<Instant>,
    pub last_git_remote_status_refresh: Instant,
    pub git_refresh_in_flight: bool,
    pub git_refresh_due_after_in_flight: bool,
    pub git_status_cache: HashMap<PathBuf, GitStatusCacheEntry>,
    pub next_resize_poll: Instant,
    pub next_animation_tick: Option<Instant>,
    pub next_auto_update_check: Option<Instant>,
    pub next_agent_manifest_update_check: Option<Instant>,
    pub agent_metadata_deadline: Option<Instant>,
    pub pending_agent_resume_deadline: Option<Instant>,
    pub selection_autoscroll_deadline: Option<Instant>,
    pub selection_highlight_clear_deadline: Option<Instant>,
    pub session_save_deadline: Option<Instant>,
    pub last_render_at: Option<Instant>,
    pub spinner_tick: u64,
    pub suppressed_repeat_keys: HashMap<String, ()>,
    pub render_notify: Arc<tokio::sync::Notify>,
    pub render_dirty: Arc<AtomicBool>,
    pub full_redraw_pending: bool,
    pub overlay_panes: HashMap<PaneId, crate::PopupPaneState>,
    pub local_terminal_notifications: bool,
    pub local_input_source_switch: bool,
    pub config_reloaded_from_disk: bool,
    pub persist_pane_history: bool,
    pub update_version_check_enabled: bool,
    pub update_manifest_check_enabled: bool,
    pub loaded_host_cursor: String,
    pub last_terminal_size: Option<(u16, u16)>,
    pub last_sidebar_divider_click: Option<Instant>,
    pub last_pane_click: Option<PaneClickState>,
    pub pending_api_worktree_creates: HashMap<PathBuf, u64>,
    pub pending_api_worktree_removes: HashMap<String, u64>,
    pub pending_api_worktree_remove_paths: HashMap<PathBuf, u64>,
    pub next_api_worktree_operation_id: u64,
    pub session_save_thread: Option<std::thread::JoinHandle<()>>,
    pub detached_custom_command_children: Vec<std::process::Child>,
}

impl AppTiming {
    pub fn new() -> Self {
        Self {
            config_diagnostic_deadline: None,
            toast_deadline: None,
            copy_feedback_deadline: None,
            last_api_notification_at: None,
            last_git_remote_status_refresh: Instant::now(),
            git_refresh_in_flight: false,
            git_refresh_due_after_in_flight: false,
            git_status_cache: HashMap::new(),
            next_resize_poll: Instant::now(),
            next_animation_tick: None,
            next_auto_update_check: None,
            next_agent_manifest_update_check: None,
            agent_metadata_deadline: None,
            pending_agent_resume_deadline: None,
            selection_autoscroll_deadline: None,
            selection_highlight_clear_deadline: None,
            session_save_deadline: None,
            last_render_at: None,
            spinner_tick: 0,
            suppressed_repeat_keys: HashMap::new(),
            render_notify: Arc::new(tokio::sync::Notify::new()),
            render_dirty: Arc::new(AtomicBool::new(false)),
            full_redraw_pending: false,
            overlay_panes: HashMap::new(),
            local_terminal_notifications: true,
            local_input_source_switch: true,
            config_reloaded_from_disk: false,
            persist_pane_history: true,
            update_version_check_enabled: true,
            update_manifest_check_enabled: true,
            loaded_host_cursor: String::new(),
            last_terminal_size: None,
            last_sidebar_divider_click: None,
            last_pane_click: None,
            pending_api_worktree_creates: HashMap::new(),
            pending_api_worktree_removes: HashMap::new(),
            pending_api_worktree_remove_paths: HashMap::new(),
            next_api_worktree_operation_id: 0,
            session_save_thread: None,
            detached_custom_command_children: Vec::new(),
        }
    }
}

impl Default for AppTiming {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PaneClickState {
    pub pane_id: PaneId,
    pub viewport_row: u16,
    pub col: u16,
    pub at: Instant,
}

impl PaneClickState {
    pub fn is_double_click_for(self, next: Self) -> bool {
        self.pane_id == next.pane_id
            && next.at.duration_since(self.at) <= std::time::Duration::from_millis(350)
            && self.viewport_row.abs_diff(next.viewport_row) <= 1
            && self.col.abs_diff(next.col) <= 1
    }
}
