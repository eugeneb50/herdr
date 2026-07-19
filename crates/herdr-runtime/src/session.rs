use herdr_state::AppState;

pub fn mark_session_dirty(state: &mut AppState) {
    state.session_dirty = true;
}

pub fn sync_session_save_schedule(state: &mut AppState) -> bool {
    if state.session_dirty {
        state.session_dirty = false;
        true
    } else {
        false
    }
}
