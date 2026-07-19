use herdr_state::{AppState, PaneId, TerminalId};

pub fn sync_terminal_titles(
    state: &mut AppState,
    get_titles: impl Fn(&TerminalId) -> Option<(Option<String>, Option<String>)>,
    on_pane_updated: impl Fn(usize, PaneId),
) -> bool {
    let mut observations = Vec::new();
    for (ws_idx, workspace) in state.workspaces.iter().enumerate() {
        for tab in &workspace.tabs {
            for (pane_id, pane) in &tab.panes {
                let Some((raw, stripped)) = get_titles(&pane.attached_terminal_id) else {
                    continue;
                };
                observations.push((
                    ws_idx,
                    *pane_id,
                    pane.attached_terminal_id.clone(),
                    raw,
                    stripped,
                ));
            }
        }
    }

    let mut raw_changed = false;
    let mut publish = Vec::new();
    for (ws_idx, pane_id, terminal_id, raw, stripped) in observations {
        let Some(terminal) = state.terminals.get_mut(&terminal_id) else {
            continue;
        };
        let change = terminal.set_terminal_title(raw, stripped);
        raw_changed |= change.raw_changed;
        if change.stripped_changed {
            publish.push((ws_idx, pane_id));
        }
    }

    for (ws_idx, pane_id) in publish {
        on_pane_updated(ws_idx, pane_id);
    }

    raw_changed
}
