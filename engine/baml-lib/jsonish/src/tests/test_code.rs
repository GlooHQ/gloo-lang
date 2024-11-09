// examples of code the LLM may generate that we need to fix

use super::*;

const BAML_FILE: &str = r#"
class Test {
    type "code"
    code string
}
"#;

test_deserializer!(
    backticks,
    BAML_FILE,
    r#"
    {
      "type": "code",
      "code": `print("Hello, world!")`
    }
    "#,
    FieldType::class("Test"),
    {
      "type": "code",
      "code": "print(\"Hello, world!\")"
    }
);

test_deserializer!(
    single_quotes,
    BAML_FILE,
    r#"
    {
      "type": "code",
      "code": 'print("Hello, world!")'
    }
    "#,
    FieldType::class("Test"),
    {
      "type": "code",
      "code": "print(\"Hello, world!\")"
    }
);

test_deserializer!(
    double_quotes,
    BAML_FILE,
    r#"
    {
      "type": "code",
      "code": "print(\"Hello, world!\")"
    }
    "#,
    FieldType::class("Test"),
    {
      "type": "code",
      "code": "print(\"Hello, world!\")"
    }
);

test_deserializer!(
    unquoted_string,
    BAML_FILE,
    r#"
    {
      "type": "code",
      "code": "print(\"Hello, world!\")"
    }
    "#,
    FieldType::class("Test"),
    {
      "type": "code",
      "code": "print(\"Hello, world!\")"
    }
);

test_deserializer!(
    triple_quotes,
    BAML_FILE,
    r#"
    {
      "type": "code",
      "code": """print("Hello, world!")"""
    }
    "#,
    FieldType::class("Test"),
    {
      "type": "code",
      "code": "print(\"Hello, world!\")"
    }
);

// Now super degenerate cases

test_deserializer!(
    unescaped_newline_double_quotes,
    BAML_FILE,
    r#"
    {
      "type": "code",
      "code": "print(\"Hello, world!
Goodbye, world!\")"
    }
    "#,
    FieldType::class("Test"),
    {
      "type": "code",
      "code": "print(\"Hello, world!\nGoodbye, world!\")"
    }
);

// Test case for unescaped newline in backticks
test_deserializer!(
    unescaped_newline_backticks,
    BAML_FILE,
    r#"
    {
      "type": "code",
      "code": `print("Hello, world!
Goodbye, world!")`
    }
    "#,
    FieldType::class("Test"),
    {
      "type": "code",
      "code": "print(\"Hello, world!\nGoodbye, world!\")"
    }
);

// Test case for unescaped newline in single quotes
test_deserializer!(
    unescaped_newline_single_quotes,
    BAML_FILE,
    r#"
    {
      "type": "code",
      "code": 'print("Hello, world!
Goodbye, world!")'
    }
    "#,
    FieldType::class("Test"),
    {
      "type": "code",
      "code": "print(\"Hello, world!\nGoodbye, world!\")"
    }
);

// Test case for unescaped newline in triple quotes
test_deserializer!(
    unescaped_newline_triple_quotes,
    BAML_FILE,
    r#"
    {
      "type": "code",
      "code": """print("Hello, world!
Goodbye, world!")"""
    }
    "#,
    FieldType::class("Test"),
    {
      "type": "code",
      "code": "print(\"Hello, world!\nGoodbye, world!\")"
    }
);

// Test case for unescaped double quotes in double quotes
test_deserializer!(
    unescaped_double_quotes_in_double_quotes,
    BAML_FILE,
    r#"
    {
      "type": "code",
      "code": "print("Hello, world!")"
    }
    "#,
    FieldType::class("Test"),
    {
      "type": "code",
      "code": "print(\"Hello, world!\")"
    }
);

// Test case for unescaped double quotes in backticks
test_deserializer!(
    unescaped_double_quotes_in_backticks,
    BAML_FILE,
    r#"
    {
      "type": "code",
      "code": `print("Hello, world!")`
    }
    "#,
    FieldType::class("Test"),
    {
      "type": "code",
      "code": "print(\"Hello, world!\")"
    }
);

// Test case for unescaped single quotes in single quotes
test_deserializer!(
    unescaped_single_quotes_in_single_quotes,
    BAML_FILE,
    r#"
    {
      "type": "code",
      "code": 'print('Hello, world!')'
    }
    "#,
    FieldType::class("Test"),
    {
      "type": "code",
      "code": "print('Hello, world!')"
    }
);

// Test case for unescaped double quotes in triple quotes
test_deserializer!(
    unescaped_double_quotes_in_triple_quotes,
    BAML_FILE,
    r#"
    {
      "type": "code",
      "code": """print("Hello, world!")"""
    }
    "#,
    FieldType::class("Test"),
    {
      "type": "code",
      "code": "print(\"Hello, world!\")"
    }
);

// Test case for unescaped single quotes in triple quotes
// TODO: THIS PARSES INCORRECTLY! Rare case, but should be fixed
// if a customer complains about it.
// https://github.com/BoundaryML/baml/issues/1145
test_deserializer!(
    unescaped_single_quotes_in_triple_quotes,
    BAML_FILE,
    r#"
    {
      "type": "code",
      "code": """print("""Hello, world!""")"""
    }
    "#,
    FieldType::class("Test"),
    {
      "type": "code",
      "code": "print("
    }
);

test_deserializer!(
    unescaped_backticks_in_backticks,
    BAML_FILE,
    r#"
    {
      "type": "code",
      "code": `console.log(`Hello, world!`)`
    }
    "#,
    FieldType::class("Test"),
    {
      "type": "code",
      "code": "console.log(`Hello, world!`)"
    }
);

test_deserializer!(
    large_backticks,
    BAML_FILE,
    r#"
  {
    "type": "code",
  "code": `import { query } from './_generated/server';
import { v } from 'convex/values';

export default query(async (ctx) => {
  const posts = await ctx.db
    .query('posts')
    .order('desc')
    .collect();

  const postsWithDetails = await Promise.all(
    posts.map(async (post) => {
      // Fetch author information
      const author = await ctx.db.get(post.authorId);
      if (!author) {
        throw new Error('Author not found');
      }

      // Count upvotes
      const upvotes = await ctx.db
        .query('upvotes')
        .filter((q) => q.eq(q.field('postId'), post._id))
        .collect();

      return {
        id: post._id.toString(),
        title: post.title,
        content: post.content,
        author: {
          id: author._id.toString(),
          name: author.name,
        },
        upvoteCount: upvotes.length,
        createdAt: post._creationTime.toString(),
      };
    })
  );

  return postsWithDetails;
})`
  }
  "#,
  FieldType::class("Test"),
  {
    "type": "code",
    "code": r#"import { query } from './_generated/server';
import { v } from 'convex/values';

export default query(async (ctx) => {
  const posts = await ctx.db
    .query('posts')
    .order('desc')
    .collect();

  const postsWithDetails = await Promise.all(
    posts.map(async (post) => {
      // Fetch author information
      const author = await ctx.db.get(post.authorId);
      if (!author) {
        throw new Error('Author not found');
      }

      // Count upvotes
      const upvotes = await ctx.db
        .query('upvotes')
        .filter((q) => q.eq(q.field('postId'), post._id))
        .collect();

      return {
        id: post._id.toString(),
        title: post.title,
        content: post.content,
        author: {
          id: author._id.toString(),
          name: author.name,
        },
        upvoteCount: upvotes.length,
        createdAt: post._creationTime.toString(),
      };
    })
  );

  return postsWithDetails;
})"#
  }
);
