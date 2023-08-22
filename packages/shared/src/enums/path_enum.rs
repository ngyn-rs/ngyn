/// Path Enum
///
/// This enum represents the possible types of paths that can be used in the application.
/// It can either be a Single path represented as a String or Multiple paths represented as a Vector of Strings.
pub enum Path {
    /// Represents a single path as a String
    Single(String),
    /// Represents multiple paths as a Vector of Strings
    Multiple(Vec<String>),
}

/// Implements the Debug trait for the Path enum
///
/// This implementation allows for the Path enum to be printed in the console for debugging purposes.
/// It matches on the Path enum and formats it as a string.
impl std::fmt::Debug for Path {
    /// Formats the Path enum as a string
    ///
    /// This function takes in a mutable reference to a Formatter and returns a Result.
    /// It matches on the Path enum and writes it to the Formatter.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Path::Single(s) => write!(f, "Single({})", s),
            Path::Multiple(v) => write!(f, "Multiple({:?})", v),
        }
    }
}
