use std::collections::HashMap;
use std::ptr::null;

pub enum ErrorSeverity {
    Fatal,
    Warning
}


pub struct Errors {
    errors: HashMap<String, String> // <ErrorName, severity>
}

impl Errors {
    pub fn new() -> Errors{
        Errors {
            errors: HashMap::new(),
        }
    }

    /// Created for getting Errors
    ///
    /// Example:
    /// ```
    ///
    /// if get_errors().len() > 1 {
    ///     for (error_name, severity) in &get_errors() {
    ///         println!("{}: {}", severity, error_name);
    ///     }
    /// }
    /// ```
    pub fn get_errors(&self) -> &HashMap<String, String> {
        &self.errors
    }

    /// Throw an Error
    pub fn throwError(&mut self, name: &str, severity: ErrorSeverity) {
        let mut sv;
        match severity {
            ErrorSeverity::Fatal => {sv = "Fatal"}
            ErrorSeverity::Warning => {sv = "Warning"}
        }
        self.errors.insert(String::from(name), String::from(sv));
    }
}

