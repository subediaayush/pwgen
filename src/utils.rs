pub fn sanitize_pattern(pattern: String) -> String {
    if !pattern.contains("(") {
        sanitize_pattern(format!("({}", pattern))
    } else if !pattern.contains(")") {
        sanitize_pattern(format!("{})", pattern))
    } else {
        pattern
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_sanitize_without_start_and_end() {
        let input = "snu";
        let expected = "(snu)";
        assert_eq!(sanitize_pattern(input.to_string()), expected);
    }

    #[test]
    fn test_sanitize_with_start_only() {
        let input = "(snu";
        let expected = "(snu)";
        assert_eq!(sanitize_pattern(input.to_string()), expected);
    }

    #[test]
    fn test_sanitize_with_end_only() {
        let input = "snu)";
        let expected = "(snu)";
        assert_eq!(sanitize_pattern(input.to_string()), expected);
    }

    #[test]
    fn test_sanitize_with_both() {
        let input = "(snu)";
        let expected = "(snu)";
        assert_eq!(sanitize_pattern(input.to_string()), expected);
    }
}