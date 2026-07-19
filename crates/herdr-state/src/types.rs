use ratatui::layout::{Direction, Rect};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TitleChange {
    pub raw_changed: bool,
    pub stripped_changed: bool,
}

impl Default for TitleChange {
    fn default() -> Self {
        Self {
            raw_changed: false,
            stripped_changed: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PaneId(pub u32);

impl fmt::Display for PaneId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct PaneInfo {
    pub id: PaneId,
    pub rect: Rect,
    pub title: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SplitBorder {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Selection {
    pub start: Option<(u16, u16)>,
    pub end: Option<(u16, u16)>,
    pub direction: Option<Direction>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct TerminalId(pub String);

impl fmt::Display for TerminalId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, Default)]
pub struct TerminalState {
    pub rows: u16,
    pub cols: u16,
    pub terminal_title: Option<String>,
    pub terminal_title_stripped: Option<String>,
}

impl TerminalState {
    pub fn set_terminal_title(
        &mut self,
        title: Option<String>,
        stripped: Option<String>,
    ) -> TitleChange {
        let raw_changed = self.terminal_title != title;
        let stripped_changed = self.terminal_title_stripped != stripped;
        self.terminal_title = title;
        self.terminal_title_stripped = stripped;
        TitleChange {
            raw_changed,
            stripped_changed,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PaneState {
    pub attached_terminal_id: TerminalId,
    pub title: Option<String>,
    pub terminal_title: Option<String>,
    pub terminal_title_stripped: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Workspace {
    pub id: String,
    pub tabs: Vec<Tab>,
    pub active_tab: usize,
}

#[derive(Debug, Clone, Default)]
pub struct Tab {
    pub id: String,
    pub title: Option<String>,
    pub panes: HashMap<PaneId, PaneState>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitStatusCacheEntry {
    pub fingerprint: String,
    pub branch: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Keybinds;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum NewTerminalCwdConfig {
    #[default]
    Follow,
    Home,
    Current,
    Path(String),
}

#[derive(Debug, Clone, Default)]
pub struct SoundConfig;

#[derive(Debug, Clone, Default)]
pub struct ToastConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToastDelivery {
    #[default]
    Off,
    Herdr,
    Terminal,
    System,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AgentState {
    #[default]
    Unknown,
    Idle,
    Working,
    Blocked,
}

pub use crate::popup_size::PopupSize;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum AgentPanelSort {
    #[default]
    Spaces,
    Priority,
}
