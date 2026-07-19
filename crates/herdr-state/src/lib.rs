//! herdr-state — pure data types and logic for herdr
//!
//! This crate contains no PTY, async, or I/O code. It is fully testable
//! without any external dependencies.

pub mod colors;
pub mod events;
pub mod popup_size;
pub mod schema;
pub mod state;
pub mod timing;
pub mod types;

// Re-export commonly used types
pub use colors::*;
pub use state::{
    AppState, Mode, ViewState, Palette, ToastKind, ToastNotification,
    CopyModeStyle, CopyModeState, NavigatorState, SettingsState,
    DragTarget, DragState, ContextMenuKind, ContextMenuState,
    WorktreeCreateState, WorktreeOpenState, WorktreeRemoveState,
    InstalledPluginRegistry, PluginPaneRecord, PaneGraphicsLayer,
    PopupPaneState, SelectionAutoscrollDirection, SelectionAutoscroll,
    RightClickPassthroughGesture,
    PaneId, PaneInfo, PaneState, SplitBorder, Selection, TerminalId,
    TerminalState, TitleChange, Workspace, Tab, GitStatusCacheEntry,
    Keybinds, NewTerminalCwdConfig, SoundConfig, ToastConfig, ToastDelivery,
    AgentState, PopupSize, AgentPanelSort,
};
pub use events::AppEvent;
pub use schema::*;
pub use timing::AppTiming;
