/// A lightweight library for validating and converting data types from strings.
pub struct TypeChecker;

impl TypeChecker {
    /// Checks if the given string can be parsed as an integer.
    pub fn is_int(s: &str) -> bool {
        s.parse::<i64>().is_ok()
    }

    /// Checks if the given string can be parsed as a floating-point number.
    pub fn is_float(s: &str) -> bool {
        s.parse::<f64>().is_ok()
    }

    /// Attempts to parse a string into an integer. Returns `None` if parsing fails.
    pub fn to_int(s: &str) -> Option<i64> {
        s.parse::<i64>().ok()
    }

    /// Attempts to parse a string into a floating-point number. Returns `None` if parsing fails.
    pub fn to_float(s: &str) -> Option<f64> {
        s.parse::<f64>().ok()
    }

    /// Checks if the given string can be parsed as a boolean.
    pub fn is_bool(s: &str) -> bool {
        matches!(s.to_lowercase().as_str(), "true" | "false")
    }

    /// Attempts to parse a string into a boolean. Returns `None` if parsing fails.
    pub fn to_bool(s: &str) -> Option<bool> {
        match s.to_lowercase().as_str() {
            "true" => Some(true),
            "false" => Some(false),
            _ => None,
        }
    }
}

// TESTS
#[cfg(test)]
mod tests {
    use super::TypeChecker;

    #[test]
    fn test_is_int() {
        assert!(TypeChecker::is_int("42"));
        assert!(TypeChecker::is_int("-100"));
        assert!(!TypeChecker::is_int("42.5"));
        assert!(!TypeChecker::is_int("hello"));
    }

    #[test]
    fn test_is_float() {
        assert!(TypeChecker::is_float("42.0"));
        assert!(TypeChecker::is_float("-100.5"));
        assert!(!TypeChecker::is_float("hello"));
    }

    #[test]
    fn test_to_int() {
        assert_eq!(TypeChecker::to_int("42"), Some(42));
        assert_eq!(TypeChecker::to_int("-100"), Some(-100));
        assert_eq!(TypeChecker::to_int("42.5"), None);
    }

    #[test]
    fn test_to_float() {
        assert_eq!(TypeChecker::to_float("42.0"), Some(42.0));
        assert_eq!(TypeChecker::to_float("-100.5"), Some(-100.5));
        assert_eq!(TypeChecker::to_float("hello"), None);
    }

    #[test]
    fn test_is_bool() {
        assert!(TypeChecker::is_bool("true"));
        assert!(TypeChecker::is_bool("false"));
        assert!(!TypeChecker::is_bool("yes"));
    }

    #[test]
    fn test_to_bool() {
        assert_eq!(TypeChecker::to_bool("true"), Some(true));
        assert_eq!(TypeChecker::to_bool("false"), Some(false));
        assert_eq!(TypeChecker::to_bool("yes"), None);
    }
}
