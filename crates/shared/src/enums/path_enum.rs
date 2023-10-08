/// Path Enum
///
/// This enum represents the possible types of paths that can be used in the application.
/// It can either be a Single path represented as a String or Multiple paths represented as a Vector of Strings.
#[derive(Debug, Clone)]
pub enum Path {
    /// Represents a single path as a String
    Single(String),
    /// Represents multiple paths as a Vector of Strings
    Multiple(Vec<String>),
}
