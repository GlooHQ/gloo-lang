use super::*;
use pretty_assertions::assert_eq;
use unindent::Unindent as _;

#[track_caller]
fn assert_format_eq(schema: &str, expected: &str) -> Result<()> {
    let formatted = format_schema(
        &schema.unindent().trim_end(),
        FormatOptions {
            indent_width: 2,
            fail_on_unhandled_rule: true,
        },
    )?;
    assert_eq!(expected.unindent().trim_end(), formatted);

    let formatted = format_schema(
        &formatted.unindent().trim_end(),
        FormatOptions {
            indent_width: 2,
            fail_on_unhandled_rule: true,
        },
    )?;
    assert_eq!(formatted.unindent().trim_end(), formatted);

    Ok(())
}

#[test]
fn class_containing_whitespace() -> anyhow::Result<()> {
    let actual = r#"
          class Foo {
          }

          class Foo { field1 string }

          class Foo {

            field1 string
          }

          class Foo {
              field1   string|int
          }
        "#
    .unindent()
    .trim_end()
    .to_string();

    let expected = r#"
          class Foo {}

          class Foo {
            field1 string
          }

          class Foo {
            field1 string
          }

          class Foo {
            field1 string | int
          }
        "#
    .unindent()
    .trim_end()
    .to_string();

    assert_format_eq(&actual, &expected)?;
    assert_format_eq(&expected, &expected)
}

#[test]
fn assorted_comment_styles() -> anyhow::Result<()> {
    let actual = r#"
    class Foo0 {
      lorem string    // trailing comments should be separated by two spaces
      ipsum string
    }

    class Foo1 {
       lorem string
      ipsum string
        // dolor string
    }

    class Foo2 {

        // "lorem" is a latin word
        lorem string

        // "ipsum" is a latin word
        ipsum string

    }

    class Foo3 {
      lorem string
      ipsum string
                    // Lorem ipsum dolor sit amet
      // Consectetur adipiscing elit
            // Sed do eiusmod tempor incididunt
      // Ut labore et dolore magna aliqua
        // Ut enim ad minim veniam
    }
        "#
    .unindent()
    .trim_end()
    .to_string();

    let expected = r#"
    class Foo0 {
      lorem string  // trailing comments should be separated by two spaces
      ipsum string
    }

    class Foo1 {
      lorem string
      ipsum string
      // dolor string
    }

    class Foo2 {
      // "lorem" is a latin word
      lorem string
      // "ipsum" is a latin word
      ipsum string
    }

    class Foo3 {
      lorem string
      ipsum string
      // Lorem ipsum dolor sit amet
      // Consectetur adipiscing elit
      // Sed do eiusmod tempor incididunt
      // Ut labore et dolore magna aliqua
      // Ut enim ad minim veniam
    }
        "#
    .unindent()
    .trim_end()
    .to_string();

    assert_format_eq(&actual, &expected)?;
    assert_format_eq(&expected, &expected)
}

// TODO: add tests for
// baml-format escape
// function prompts are NOT formatted
