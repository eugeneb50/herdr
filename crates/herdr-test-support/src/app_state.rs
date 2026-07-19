use herdr_state::AppState;

pub trait AppStateTestExt {
    fn test_new() -> Self;
    fn assert_invariants_for_test(&self);
}

impl AppStateTestExt for AppState {
    fn test_new() -> Self {
        Self::default()
    }

    fn assert_invariants_for_test(&self) {
        // Test invariants
    }
}
