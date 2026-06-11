// Public Interface
pub mod diagnostic_report;
pub mod input_script;
pub mod lsp;

// Syntax Tree
pub(crate) mod ast;
pub mod spans;

pub(crate) mod utils;

// Error handling
pub mod diagnostics;
pub(crate) mod spanned_error;

// Input script validation
pub(crate) mod check_commands;
pub(crate) mod check_styles;
pub(crate) mod error_finder;
pub(crate) mod identifinder;
pub(crate) mod lints;

// Hover documentation
pub(crate) mod docs;

// Command Styles
pub(crate) mod commands;
pub(crate) mod styles;

pub mod matmaster;
