use std::fmt::{Display, Formatter};
use colored::Colorize;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Severity {
    Error,
    Warning,
    Help,
}

#[derive(Debug, PartialEq)]
pub struct Diagnostic {
    severity: Severity,
    message: String,
    error_code: Option<usize>,
    preview: Option<String>,
}

impl Default for Diagnostic {
    fn default() -> Self {
        Self {
            severity: Severity::Error,
            message: String::new(),
            error_code: None,
            preview: None,
        }
    }
}

#[derive(Default)]
pub struct DiagnosticBuilder {
    inner: Diagnostic,
}

impl DiagnosticBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_severity(mut self, severity: Severity) -> Self {
        self.inner.severity = severity;
        self
    }

    pub fn add_message(mut self, message: impl Into<String>) -> Self {
        self.inner.message = message.into();
        self
    }

    pub fn with_error_code<T: crate::XynError>(mut self, error: T) -> Self {
        self.inner.message = error.message().to_string();
        self.inner.error_code = Some(error.to_usize());
        self.inner.severity = Severity::Error; 
        self
    }

    pub fn with_preview(mut self, preview: impl ToString) -> Self {
        self.inner.preview = Some(preview.to_string());
        self
    }

    pub fn build(self) -> Diagnostic {
        self.inner
    }
}

impl Display for Diagnostic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (_label, colored_label) = match self.severity {
            Severity::Error => {
                let text = if let Some(code) = self.error_code {
                    format!("error[E-{:04}]", code)
                } else {
                    "error".to_string()
                };
                (text.clone(), text.red().bold())
            }
            Severity::Warning => {
                let text = "warning".to_string();
                (text.clone(), text.yellow().bold())
            }
            Severity::Help => {
                let text = "help".to_string();
                (text.clone(), text.into())
            }
        };

        write!(f, "{}: {}", colored_label, self.message.bold())
    }
}