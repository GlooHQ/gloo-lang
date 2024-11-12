pub struct DedentedString {
    pub content: String,
    pub indent_size: usize,
}

pub fn dedent(s: &str) -> DedentedString {
    let mut prefix = "";
    let mut lines = s.lines();

    // We first search for a non-empty line to find a prefix.
    for line in &mut lines {
        let mut whitespace_idx = line.len();
        for (idx, ch) in line.char_indices() {
            if !ch.is_whitespace() {
                whitespace_idx = idx;
                break;
            }
        }

        // Check if the line had anything but whitespace
        if whitespace_idx < line.len() {
            prefix = &line[..whitespace_idx];
            break;
        }
    }

    // We then continue looking through the remaining lines to
    // possibly shorten the prefix.
    for line in &mut lines {
        let mut whitespace_idx = line.len();
        for ((idx, a), b) in line.char_indices().zip(prefix.chars()) {
            if a != b {
                whitespace_idx = idx;
                break;
            }
        }

        // Check if the line had anything but whitespace and if we
        // have found a shorter prefix
        if whitespace_idx < line.len() && whitespace_idx < prefix.len() {
            prefix = &line[..whitespace_idx];
        }
    }

    // We now go over the lines a second time to build the result.
    let mut result = String::new();
    for line in s.lines() {
        if line.starts_with(prefix) && line.chars().any(|c| !c.is_whitespace()) {
            let (_, tail) = line.split_at(prefix.len());
            result.push_str(tail);
        }
        result.push('\n');
    }

    if result.ends_with('\n') && !s.ends_with('\n') {
        let new_len = result.len() - 1;
        result.truncate(new_len);
    }

    DedentedString {
        content: result,
        indent_size: prefix.len(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_dedent() {
        let input = r#"
            hello
            world
            "#;
        let expected = r#"hello
world
"#;
        let result = dedent(input);
        assert_eq!(result.content, expected);
        assert_eq!(result.indent_size, 12);
    }

    #[test]
    fn test_mixed_indentation() {
        let input = r#"
            first line
                indented line
            back to first level
        "#;
        let expected = r#"first line
    indented line
back to first level
"#;
        let result = dedent(input);
        assert_eq!(result.content, expected);
        assert_eq!(result.indent_size, 12);
    }

    #[test]
    fn test_empty_lines() {
        let input = r#"
            line1

            line2
        "#;
        let expected = r#"line1

line2
"#;
        let result = dedent(input);
        assert_eq!(result.content, expected);
        assert_eq!(result.indent_size, 12);
    }

    #[test]
    fn test_no_indentation() {
        let input = "hello\nworld";
        let expected = "hello\nworld";
        let result = dedent(input);
        assert_eq!(result.content, expected);
        assert_eq!(result.indent_size, 0);
    }

    #[test]
    fn test_different_line_starts() {
        let input = r#"
            def function():
                # comment
                print("hello")
            "#;
        let expected = r#"def function():
    # comment
    print("hello")
"#;
        let result = dedent(input);
        assert_eq!(result.content, expected);
        assert_eq!(result.indent_size, 12);
    }

    #[test]
    fn test_tabs_and_spaces() {
        let input = r#"
		    mixed
		    indentation
		"#;
        let expected = r#"mixed
indentation
"#;
        let result = dedent(input);
        assert_eq!(result.content, expected);
        assert_eq!(result.indent_size, 7);
    }

    #[test]
    fn test_single_line() {
        let input = "    single line";
        let expected = "single line";
        let result = dedent(input);
        assert_eq!(result.content, expected);
        assert_eq!(result.indent_size, 4);
    }
}
