//! Core application state types

use crate::colors::{HostAppearance, TerminalTheme};
use crate::schema::{
    InstalledPluginInfo, PaneGraphicsFormat, PaneGraphicsPlacementParams,
};
pub use crate::types::{
    AgentPanelSort, AgentState, GitStatusCacheEntry, Keybinds,
    NewTerminalCwdConfig, PaneId, PaneInfo, PaneState, PopupSize, Selection,
    SoundConfig, SplitBorder, Tab, TerminalId, TerminalState, TitleChange,
    ToastConfig, ToastDelivery, Workspace,
};
use ratatui::layout::Rect;
use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::time::Instant;

// ---------------------------------------------------------------------------
// Palette / Theme types
// ---------------------------------------------------------------------------

/// All colors used by the UI
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Palette {
    pub bg: Color,
    pub fg: Color,
    pub panel_bg: Color,
    pub panel_border: Color,
    pub panel_border_active: Color,
    pub panel_border_inactive: Color,
    pub status_bg: Color,
    pub status_fg: Color,
    pub status_border: Color,
    pub status_border_active: Color,
    pub sidebar_bg: Color,
    pub sidebar_fg: Color,
    pub sidebar_border: Color,
    pub tab_bg: Color,
    pub tab_fg: Color,
    pub tab_active_bg: Color,
    pub tab_active_fg: Color,
    pub tab_border: Color,
    pub scrollbar_bg: Color,
    pub scrollbar_thumb: Color,
    pub selection_bg: Color,
    pub selection_fg: Color,
    pub cursor_bg: Color,
    pub cursor_fg: Color,
    pub prompt_bg: Color,
    pub prompt_fg: Color,
    pub prompt_border: Color,
    pub toast_bg: Color,
    pub toast_fg: Color,
    pub toast_border: Color,
    pub done: Color,
    pub working: Color,
    pub blocked: Color,
    pub idle: Color,
    pub unknown: Color,
    pub accent: Color,
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            bg: Color::Reset,
            fg: Color::Reset,
            panel_bg: Color::Reset,
            panel_border: Color::Reset,
            panel_border_active: Color::Reset,
            panel_border_inactive: Color::Reset,
            status_bg: Color::Reset,
            status_fg: Color::Reset,
            status_border: Color::Reset,
            status_border_active: Color::Reset,
            sidebar_bg: Color::Reset,
            sidebar_fg: Color::Reset,
            sidebar_border: Color::Reset,
            tab_bg: Color::Reset,
            tab_fg: Color::Reset,
            tab_active_bg: Color::Reset,
            tab_active_fg: Color::Reset,
            tab_border: Color::Reset,
            scrollbar_bg: Color::Reset,
            scrollbar_thumb: Color::Reset,
            selection_bg: Color::Reset,
            selection_fg: Color::Reset,
            cursor_bg: Color::Reset,
            cursor_fg: Color::Reset,
            prompt_bg: Color::Reset,
            prompt_fg: Color::Reset,
            prompt_border: Color::Reset,
            toast_bg: Color::Reset,
            toast_fg: Color::Reset,
            toast_border: Color::Reset,
            done: Color::Green,
            working: Color::Yellow,
            blocked: Color::Red,
            idle: Color::Blue,
            unknown: Color::Gray,
            accent: Color::Magenta,
        }
    }
}

// ---------------------------------------------------------------------------
// Mode / View State
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    #[default]
    Normal,
    Command,
    Search,
    Copy,
    Navigator,
    Settings,
    WorktreeCreate,
    WorktreeOpen,
    WorktreeRemove,
    AgentRename,
    Popup,
    #[serde(skip)]
    Widget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ViewState {
    #[default]
    Panes,
    Sidebar,
    Navigator,
    Settings,
    WorktreeCreate,
    WorktreeOpen,
    WorktreeRemove,
    AgentRename,
    CopyMode,
    Popup,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToastKind {
    Info,
    Warn,
    Error,
    Done,
}

#[derive(Debug, Clone)]
pub struct ToastNotification {
    pub kind: ToastKind,
    pub title: String,
    pub body: Option<String>,
    pub deadline: Instant,
}

impl ToastNotification {
    pub fn new(kind: ToastKind, title: String, body: Option<String>, duration: std::time::Duration) -> Self {
        Self {
            kind,
            title,
            body,
            deadline: Instant::now() + duration,
        }
    }
}

// ---------------------------------------------------------------------------
// Copy Mode
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum CopyModeStyle {
    #[default]
    Block,
    Line,
    Word,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CopyModeState {
    pub selection: Selection,
    pub style: CopyModeStyle,
    pub search_query: String,
    pub search_matches: Vec<Rect>,
    pub search_current: Option<usize>,
}

// ---------------------------------------------------------------------------
// Navigator
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NavigatorState {
    pub query: String,
    pub filtered_workspaces: Vec<usize>,
    pub selected_idx: Option<usize>,
    pub last_workspace_count: usize,
}

// ---------------------------------------------------------------------------
// Settings
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SettingsState {
    pub active_section: usize,
    pub active_row: usize,
    pub filter: String,
    pub filtered_items: Vec<(usize, String)>,
}

// ---------------------------------------------------------------------------
// Drag / Context Menu / Worktree
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum DragTarget {
    #[default]
    None,
    Pane(PaneId),
    TabBar { tab_idx: usize, insert_before: bool },
    WorkspaceBar { ws_idx: usize, insert_before: bool },
    SidebarDivider,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DragState {
    pub target: DragTarget,
    pub start_col: u16,
    pub start_row: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ContextMenuKind {
    #[default]
    Pane,
    Tab,
    Workspace,
    Sidebar,
    Agent,
    Worktree,
    Settings,
    Empty,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContextMenuState {
    pub kind: ContextMenuKind,
    pub anchor_col: u16,
    pub anchor_row: u16,
    pub items: Vec<String>,
    pub selected: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorktreeCreateState {
    pub repo_path: String,
    pub branch: String,
    pub path: String,
    pub focus: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorktreeOpenState {
    pub search: String,
    pub filtered: Vec<String>,
    pub selected: Option<usize>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorktreeRemoveState {
    pub search: String,
    pub filtered: Vec<String>,
    pub selected: Option<usize>,
}

// ---------------------------------------------------------------------------
// Plugin / Graphics
// ---------------------------------------------------------------------------

pub type InstalledPluginRegistry = HashMap<String, InstalledPluginInfo>;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PluginPaneRecord {
    pub plugin_id: String,
    pub entrypoint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaneGraphicsLayer {
    pub format: PaneGraphicsFormat,
    pub image_width: u32,
    pub image_height: u32,
    pub data: Vec<u8>,
    pub data_fingerprint: u64,
    pub render: PaneGraphicsPlacementParams,
}

impl PaneGraphicsLayer {
    pub(crate) fn new(
        format: PaneGraphicsFormat,
        image_width: u32,
        image_height: u32,
        data: Vec<u8>,
        render: PaneGraphicsPlacementParams,
    ) -> Self {
        let data_fingerprint = pane_graphics_data_fingerprint(&data);
        Self {
            format,
            image_width,
            image_height,
            data,
            data_fingerprint,
            render,
        }
    }
}

fn pane_graphics_data_fingerprint(data: &[u8]) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    data.hash(&mut hasher);
    hasher.finish()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopupPaneState {
    pub pane_id: PaneId,
    pub terminal_id: TerminalId,
    pub width: Option<PopupSize>,
    pub height: Option<PopupSize>,
}

// ---------------------------------------------------------------------------
// Selection Autoscroll
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectionAutoscrollDirection {
    Up,
    Down,
}

#[derive(Clone, Debug)]
pub struct SelectionAutoscroll {
    pub direction: SelectionAutoscrollDirection,
    pub last_mouse_screen_col: u16,
    pub last_mouse_screen_row: u16,
    pub inner_rect: Rect,
}

#[derive(Clone)]
pub struct RightClickPassthroughGesture {
    pub pane_info: PaneInfo,
    pub modifiers: crossterm::event::KeyModifiers,
}

// ---------------------------------------------------------------------------
// Main AppState
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub terminals: HashMap<TerminalId, TerminalState>,
    pub workspaces: Vec<Workspace>,
    pub active_workspace_idx: usize,
    pub mode: Mode,
    pub view_state: ViewState,
    pub palette: Palette,
    pub toast: Option<ToastNotification>,
    pub copy_mode: CopyModeState,
    pub navigator: NavigatorState,
    pub settings: SettingsState,
    pub context_menu: ContextMenuState,
    pub drag_state: DragState,
    pub worktree_create: WorktreeCreateState,
    pub worktree_open: WorktreeOpenState,
    pub worktree_remove: WorktreeRemoveState,
    pub plugin_registry: InstalledPluginRegistry,
    pub plugin_panes: Vec<PluginPaneRecord>,
    pub pane_graphics_layers: HashMap<PaneId, Vec<PaneGraphicsLayer>>,
    pub popup_panes: HashMap<PaneId, PopupPaneState>,
    pub keybinds: Keybinds,
    pub new_terminal_cwd: NewTerminalCwdConfig,
    pub sound: SoundConfig,
    pub toast_config: ToastConfig,
    pub show_agent_labels_on_pane_borders: bool,
    pub pane_history_persistence: bool,
    pub switch_ascii_input_source_in_prefix: bool,
    pub terminal_theme: TerminalTheme,
    pub host_appearance: Option<HostAppearance>,
    pub host_appearance_explicit: bool,
    pub theme_name: String,
    pub config_diagnostic: Option<String>,
    pub session_dirty: bool,
    pub active: Option<usize>,
    pub selected: usize,
    pub sidebar_width: u16,
    pub sidebar_section_split: f32,
    pub collapsed_space_keys: std::collections::HashSet<String>,
}

impl AppState {
    pub fn test_new() -> Self {
        Self::default()
    }

    pub fn assert_invariants_for_test(&self) {
        // Test invariants
    }
}

