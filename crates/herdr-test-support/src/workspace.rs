use herdr_state::types::Workspace;

pub trait WorkspaceTestExt {
    fn test_new() -> Self;
    fn assert_invariants_for_test(&self);
}

impl WorkspaceTestExt for Workspace {
    fn test_new() -> Self {
        Self::default()
    }

    fn assert_invariants_for_test(&self) {
        // Test invariants
    }
}
