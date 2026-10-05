use crate::entities::Step;

/// Describes how an action is represented and executed.
#[derive(Debug, Clone, PartialEq)]
pub enum ActionRuntime {
    /// Composite action: runs shell steps in the job's container.
    Composite {
        /// Steps to execute sequentially.
        steps: Vec<Step>,
    },

    /// Node action: runs a JavaScript file (node12 variant).
    Node12 {
        /// Entry point script.
        main: String,
    },

    /// Node action: runs a JavaScript file (node16 variant).
    Node16 {
        /// Entry point script.
        main: String,
    },

    /// Node action (node20 variant).
    Node20 {
        /// Entry point script.
        main: String,
    },

    /// Docker action identified by its image; execution currently reports an
    /// unsupported-action error.
    Docker {
        /// Docker image to run.
        image: String,
    },
}
