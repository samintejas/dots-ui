//! Shared enums used across components.

/// A state a component can report. Never decoration: see the design system's colour rules.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Severity {
    Info,
    Success,
    Warning,
    Danger,
}

impl Severity {
    /// The value of `data-severity`.
    pub fn as_str(self) -> &'static str {
        match self {
            Severity::Info => "info",
            Severity::Success => "success",
            Severity::Warning => "warning",
            Severity::Danger => "danger",
        }
    }

    /// The Lucide icon whose outline tells this severity apart without colour.
    pub fn icon(self) -> &'static str {
        match self {
            Severity::Info => "info",
            Severity::Success => "circle-check",
            Severity::Warning => "triangle-alert",
            Severity::Danger => "octagon-alert",
        }
    }
}

/// Control height: `Sm` is `control-h-sm` (28px), `Md` the default `control-h` (32px), `Lg` `control-h-lg` (40px).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Size {
    Sm,
    #[default]
    Md,
    Lg,
}
