//! EntityScope: A tool for exploring and generating entity data

#[cfg(feature = "generator")]
pub use entity_scope_generator;

#[cfg(feature = "explorer")]
pub use entity_scope_explorer;

#[cfg(feature = "runtime")]
pub use entity_twin_runtime;

// You can add common functionality here

/// Main entry point for EntityScope functionality
pub struct EntityScope {
    // Add fields as needed
}

impl EntityScope {
    /// Create a new EntityScope instance
    pub(crate) fn new() -> Self {
        // Implementation
        Self {}
    }

    // Add methods that use features conditionally
    #[cfg(feature = "generator")]
    pub fn generate(&self) {
        // Use entity_scope_generator functionality
    }

    #[cfg(feature = "explorer")]
    pub fn explore(&self) {
        // Use entity_scope_explorer functionality
    }

    #[cfg(feature = "runtime")]
    pub fn run(&self) {
        // Use entity_twin_runtime functionality
    }
}

impl Default for EntityScope {
    fn default() -> Self {
        Self::new()
    }
}
