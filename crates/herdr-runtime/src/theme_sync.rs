use herdr_state::{
    AppState, DefaultColorKind, HostAppearance, Palette, RgbColor, TerminalTheme,
};

pub fn query_host_terminal_theme() {
    use std::io::Write;
    let _ = std::io::stdout()
        .write_all(herdr_state::colors::HOST_COLOR_QUERY_SEQUENCE.as_bytes());
    let _ = std::io::stdout().flush();
}

pub fn update_host_terminal_theme(
    state: &mut AppState,
    kind: DefaultColorKind,
    color: RgbColor,
) -> bool {
    let mut changed = false;
    if matches!(kind, DefaultColorKind::Background) && !state.host_appearance_explicit {
        changed |= set_host_terminal_appearance(state, color.inferred_appearance(), false);
    }
    let next_theme = state.terminal_theme.with_color(kind, color);
    changed | set_host_terminal_theme(state, next_theme)
}

pub fn set_host_terminal_appearance(
    state: &mut AppState,
    appearance: HostAppearance,
    explicit: bool,
) -> bool {
    if state.host_appearance == Some(appearance) && state.host_appearance_explicit == explicit {
        return false;
    }
    if state.host_appearance_explicit && !explicit {
        return false;
    }
    state.host_appearance = Some(appearance);
    state.host_appearance_explicit = explicit;
    true
}

pub fn set_host_terminal_appearance_state(
    state: &mut AppState,
    appearance: Option<HostAppearance>,
    explicit: bool,
) -> bool {
    if state.host_appearance == appearance && state.host_appearance_explicit == explicit {
        return false;
    }
    state.host_appearance = appearance;
    state.host_appearance_explicit = explicit;
    true
}

pub fn set_host_terminal_theme(state: &mut AppState, theme: TerminalTheme) -> bool {
    if theme == state.terminal_theme {
        return false;
    }
    state.terminal_theme = theme;
    true
}

pub fn refresh_effective_app_theme(
    state: &mut AppState,
    palette: Palette,
    theme_name: String,
) -> bool {
    if state.theme_name == theme_name && state.palette == palette {
        return false;
    }
    state.theme_name = theme_name;
    state.palette = palette;
    true
}
