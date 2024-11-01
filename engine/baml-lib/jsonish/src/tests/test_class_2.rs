use baml_types::FieldType;

use super::*;

const TEST_FILE: &str = r###"
class ServerActionTask {
    type "server_action" @description(#"
        A server action is a function that will be generated by the codegen agent.
    "#)
    name string
    description string
    signature string @alias("function_signature")
}

class PageTask {
    type "page"
    name string
    description string
    required_components string[]
    required_actions string[]
    route string
}

class ComponentTask {
    type "component"
    name string
    description string
    props string
}

class Task {
    tasks (ServerActionTask | PageTask | ComponentTask)[]
}
"###;

test_deserializer!(
  test_single_page_task,
  TEST_FILE,
  r#"
{
      type: page,
      name: HomePage,
      description: Landing page with post list,
      required_components: [PostCard, PostFilter],
      required_actions: [fetchPosts],
      route: /
}  
  "#,
  FieldType::Class("PageTask".to_string()),
  {
    "type": "page",
    "name": "HomePage",
    "description": "Landing page with post list",
    "required_components": ["PostCard", "PostFilter"],
    "required_actions": ["fetchPosts"],
    "route": "/"
  }
);

test_deserializer!(
  test_class_2_single,
  TEST_FILE,
  r#"[
    {
      type: server_action,
      name: fetchPosts,
      description: Fetch paginated blog posts with sorting and filtering,
      function_signature: async function fetchPosts(page: number, sort: string, filters: object): Promise<PostList>
    }
  ]"#,
  FieldType::union(vec![
    FieldType::Class("ServerActionTask".to_string()),
    FieldType::Class("PageTask".to_string()),
    FieldType::Class("ComponentTask".to_string()),
  ]).as_list(),
  [
    {
      "type": "server_action",
      "name": "fetchPosts",
      "description": "Fetch paginated blog posts with sorting and filtering",
      "signature": "async function fetchPosts(page: number, sort: string, filters: object): Promise<PostList>"
    }
  ]
);

test_deserializer!(
  test_class_2_two,
  TEST_FILE,
  r#"[
    {
      type: server_action,
      name: fetchPosts,
      description: Fetch paginated blog posts with sorting and filtering,
      function_signature: async function fetchPosts(page: number, sort: string, filters: object): Promise<PostList>
    },
    {
      type: component,
      name: PostCard,
      description: Card component for displaying post preview on home page,
      props: {title: string, excerpt: string, author: Author, date: string, onClick: () => void}
    }
  ]"#,
  FieldType::union(vec![
    FieldType::Class("ServerActionTask".to_string()),
    FieldType::Class("PageTask".to_string()),
    FieldType::Class("ComponentTask".to_string()),
  ]).as_list(),
  [
    {
      "type": "server_action",
      "name": "fetchPosts",
      "description": "Fetch paginated blog posts with sorting and filtering",
      "signature": "async function fetchPosts(page: number, sort: string, filters: object): Promise<PostList>"
    },
    {
      "type": "component",
      "name": "PostCard",
      "description": "Card component for displaying post preview on home page",
      "props": "{title: string, excerpt: string, author: Author, date: string, onClick: () => void}"
    }
  ]
);

test_deserializer!(
  test_class_2_three,
  TEST_FILE,
  r#"[
    {
      type: server_action,
      name: fetchPosts,
      description: Fetch paginated blog posts with sorting and filtering,
      function_signature: async function fetchPosts(page: number, sort: string, filters: object): Promise<PostList>
    },
    {
      type: component,
      name: PostCard,
      description: Card component for displaying post preview on home page,
      props: {title: string, excerpt: string, author: Author, date: string, onClick: () => void}
    },
    {
      type: page,
      name: HomePage,
      description: Landing page with post list,
      required_components: [PostCard, PostFilter],
      required_actions: [fetchPosts],
      route: /
    }
  ]"#,
  FieldType::union(vec![
    FieldType::Class("ServerActionTask".to_string()),
    FieldType::Class("PageTask".to_string()),
    FieldType::Class("ComponentTask".to_string()),
  ]).as_list(),
  [
    {
      "type": "server_action",
      "name": "fetchPosts",
      "description": "Fetch paginated blog posts with sorting and filtering",
      "signature": "async function fetchPosts(page: number, sort: string, filters: object): Promise<PostList>"
    },
    {
      "type": "component",
      "name": "PostCard",
      "description": "Card component for displaying post preview on home page",
      "props": "{title: string, excerpt: string, author: Author, date: string, onClick: () => void}"
    },
    {
      "type": "page",
      "name": "HomePage",
      "description": "Landing page with post list",
      "required_components": ["PostCard", "PostFilter"],
      "required_actions": ["fetchPosts"],
      "route": "/"
    }
  ]
);

test_deserializer!(
  test_class_2_four,
  TEST_FILE,
  r#"[
    {
      type: server_action,
      name: fetchPosts,
      description: Fetch paginated blog posts with sorting and filtering,
      function_signature: async function fetchPosts(page: number, sort: string, filters: object): Promise<PostList>
    },
    {
      type: component,
      name: PostCard,
      description: Card component for displaying post preview on home page,
      props: {title: string, excerpt: string, author: Author, date: string, onClick: () => void}
    },
    {
      type: page,
      name: HomePage,
      description: Landing page with post list,
      required_components: [PostCard, PostFilter],
      required_actions: [fetchPosts],
      route: /
    },
    {
      type: server_action,
      name: fetchPostById,
      description: Fetch single post with full content and metadata,
      function_signature: async function fetchPostById(id: string): Promise<Post>
    }
  ]"#,
  FieldType::union(vec![
    FieldType::Class("ServerActionTask".to_string()),
    FieldType::Class("PageTask".to_string()),
    FieldType::Class("ComponentTask".to_string()),
  ]).as_list(),
  [
    {
      "type": "server_action",
      "name": "fetchPosts",
      "description": "Fetch paginated blog posts with sorting and filtering",
      "signature": "async function fetchPosts(page: number, sort: string, filters: object): Promise<PostList>"
    },
    {
      "type": "component",
      "name": "PostCard",
      "description": "Card component for displaying post preview on home page",
      "props": "{title: string, excerpt: string, author: Author, date: string, onClick: () => void}"
    },
    {
      "type": "page",
      "name": "HomePage",
      "description": "Landing page with post list",
      "required_components": ["PostCard", "PostFilter"],
      "required_actions": ["fetchPosts"],
      "route": "/"
    },
    {
      "type": "server_action",
      "name": "fetchPostById",
      "description": "Fetch single post with full content and metadata",
      "signature": "async function fetchPostById(id: string): Promise<Post>"
    }
  ]
);
