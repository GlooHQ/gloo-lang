use std::sync::Arc;

use anyhow::Result;
use baml_types::{Constraint, FieldType, TypeValue};
use indexmap::{IndexMap, IndexSet};

#[derive(Debug)]
pub struct Name {
    name: String,
    rendered_name: Option<String>,
}

impl Name {
    pub fn new(name: String) -> Self {
        Self {
            name,
            rendered_name: None,
        }
    }

    pub fn new_with_alias(name: String, alias: Option<String>) -> Self {
        Self {
            name,
            rendered_name: alias,
        }
    }

    pub fn rendered_name(&self) -> &str {
        self.rendered_name.as_ref().unwrap_or(&self.name)
    }

    pub fn real_name(&self) -> &str {
        &self.name
    }
}

// TODO: (Greg) Enum needs to carry its constraints.
#[derive(Debug)]
pub struct Enum {
    pub name: Name,
    // name and description
    pub values: Vec<(Name, Option<String>)>,
    pub constraints: Vec<Constraint>,
}

/// The components of a Class needed to render `OutputFormatContent`.
/// This type is also used by `jsonish` to drive flexible parsing.
#[derive(Debug)]
pub struct Class {
    pub name: Name,
    // fields have name, type and description.
    pub fields: Vec<(Name, FieldType, Option<String>)>,
    pub constraints: Vec<Constraint>,
}

#[derive(Debug, Clone)]
pub struct OutputFormatContent {
    enums: Arc<IndexMap<String, Enum>>,
    classes: Arc<IndexMap<String, Class>>,
    recursive_classes: Arc<IndexSet<String>>,
    target: FieldType,
}

/// Builder for [`OutputFormatContent`].
pub struct Builder {
    enums: Vec<Enum>,
    classes: Vec<Class>,
    /// Order matters for this one.
    recursive_classes: IndexSet<String>,
    target: FieldType,
}

impl Builder {
    pub fn new(target: FieldType) -> Self {
        Self {
            enums: vec![],
            classes: vec![],
            recursive_classes: IndexSet::new(),
            target,
        }
    }

    pub fn enums(mut self, enums: Vec<Enum>) -> Self {
        self.enums = enums;
        self
    }

    pub fn classes(mut self, classes: Vec<Class>) -> Self {
        self.classes = classes;
        self
    }

    pub fn recursive_classes(mut self, recursive_classes: IndexSet<String>) -> Self {
        self.recursive_classes = recursive_classes;
        self
    }

    pub fn target(mut self, target: FieldType) -> Self {
        self.target = target;
        self
    }

    pub fn build(self) -> OutputFormatContent {
        OutputFormatContent {
            enums: Arc::new(
                self.enums
                    .into_iter()
                    .map(|e| (e.name.name.clone(), e))
                    .collect(),
            ),
            classes: Arc::new(
                self.classes
                    .into_iter()
                    .map(|c| (c.name.name.clone(), c))
                    .collect(),
            ),
            recursive_classes: Arc::new(self.recursive_classes.into_iter().collect()),
            target: self.target,
        }
    }
}

enum RenderSetting<T> {
    Auto,
    Always(T),
    Never,
}

impl<T> Default for RenderSetting<T> {
    fn default() -> Self {
        Self::Auto
    }
}

#[derive(strum::EnumString, strum::VariantNames)]
pub(crate) enum MapStyle {
    #[strum(serialize = "angle")]
    TypeParameters,

    #[strum(serialize = "object")]
    ObjectLiteral,
}

pub(crate) struct RenderOptions {
    prefix: RenderSetting<String>,
    pub(crate) or_splitter: String,
    enum_value_prefix: RenderSetting<String>,
    hoisted_class_prefix: String,
    always_hoist_enums: RenderSetting<bool>,
    map_style: MapStyle,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            prefix: RenderSetting::Auto,
            or_splitter: Self::DEFAULT_OR_SPLITTER.to_string(),
            enum_value_prefix: RenderSetting::Auto,
            hoisted_class_prefix: Self::DEFAULT_HOISTED_CLASS_PREFIX.to_string(),
            always_hoist_enums: RenderSetting::Auto,
            map_style: MapStyle::TypeParameters,
        }
    }
}

impl RenderOptions {
    const DEFAULT_OR_SPLITTER: &'static str = " or ";
    const DEFAULT_HOISTED_CLASS_PREFIX: &'static str = "";

    pub(crate) fn new(
        prefix: Option<Option<String>>,
        or_splitter: Option<String>,
        enum_value_prefix: Option<Option<String>>,
        always_hoist_enums: Option<bool>,
        map_style: Option<MapStyle>,
        hoisted_class_prefix: Option<String>,
    ) -> Self {
        Self {
            prefix: prefix.map_or(RenderSetting::Auto, |p| {
                p.map_or(RenderSetting::Never, RenderSetting::Always)
            }),
            or_splitter: or_splitter.unwrap_or(Self::DEFAULT_OR_SPLITTER.to_string()),
            enum_value_prefix: enum_value_prefix.map_or(RenderSetting::Auto, |p| {
                p.map_or(RenderSetting::Never, RenderSetting::Always)
            }),
            always_hoist_enums: always_hoist_enums
                .map_or(RenderSetting::Auto, RenderSetting::Always),
            map_style: map_style.unwrap_or(MapStyle::TypeParameters),
            hoisted_class_prefix: hoisted_class_prefix
                .unwrap_or(Self::DEFAULT_HOISTED_CLASS_PREFIX.to_string()),
        }
    }
}

struct Attribute {
    name: String,
    description: Option<String>,
}

struct EnumRender {
    name: String,
    delimiter: String,
    values: Vec<Attribute>,
}

impl EnumRender {
    fn to_string(&self, options: &RenderOptions) -> String {
        let mut result = format!("{}\n{}", self.name, self.delimiter);
        for value in &self.values {
            result.push_str(&format!(
                "\n{}{}",
                match options.enum_value_prefix {
                    RenderSetting::Auto => "- ",
                    RenderSetting::Always(ref prefix) => prefix,
                    RenderSetting::Never => "",
                },
                value.to_string()
            ));
        }
        result
    }
}

impl Attribute {
    fn to_string(&self) -> String {
        if let Some(description) = &self.description {
            format!("{}: {}", self.name, description.replace("\n", "\n  "))
        } else {
            self.name.clone()
        }
    }
}

struct ClassRender {
    #[allow(dead_code)]
    name: String,
    values: Vec<ClassFieldRender>,
}

struct ClassFieldRender {
    name: String,
    r#type: String,
    description: Option<String>,
}

impl std::fmt::Display for ClassRender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{{")?;
        for value in &self.values {
            if let Some(desc) = &value.description {
                writeln!(f, "  // {}", desc.replace("\n", "\n  // "))?;
            }
            writeln!(
                f,
                "  {}: {},",
                value.name,
                value.r#type.replace('\n', "\n  ")
            )?;
        }
        write!(f, "}}")
    }
}

struct MapRender<'s> {
    style: &'s MapStyle,
    key_type: String,
    value_type: String,
}

impl<'s> std::fmt::Display for MapRender<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.style {
            MapStyle::TypeParameters => write!(f, "map<{}, {}>", self.key_type, self.value_type),
            MapStyle::ObjectLiteral => write!(f, "{{{}: {}}}", self.key_type, self.value_type),
        }
    }
}

struct RenderState {
    hoisted_enums: IndexSet<String>,
    hoisted_classes: IndexSet<String>,
}

impl OutputFormatContent {
    pub fn target(target: FieldType) -> Builder {
        Builder::new(target)
    }

    fn prefix<'a>(&self, options: &'a RenderOptions) -> Option<&'a str> {
        fn auto_prefix(
            ft: &FieldType,
            output_format_content: &OutputFormatContent,
        ) -> Option<&'static str> {
            match ft {
                FieldType::Primitive(TypeValue::String) => None,
                FieldType::Primitive(_) => Some("Answer as a: "),
                FieldType::Literal(_) => Some("Answer using this specific value:\n"),
                FieldType::Enum(_) => Some("Answer with any of the categories:\n"),
                // TODO: Func returns &str we can't format!, do something to
                // avoid duplicating the string.
                FieldType::Class(cls) => {
                    Some(if output_format_content.recursive_classes.contains(cls) {
                        "Answer in JSON using this schema: "
                    } else {
                        "Answer in JSON using this schema:\n"
                    })
                }
                FieldType::List(_) => Some("Answer with a JSON Array using this schema:\n"),
                FieldType::Union(_) => Some("Answer in JSON using any of these schemas:\n"),
                FieldType::Optional(_) => Some("Answer in JSON using this schema:\n"),
                FieldType::Map(_, _) => Some("Answer in JSON using this schema:\n"),
                FieldType::Tuple(_) => None,
                FieldType::Constrained { base, .. } => auto_prefix(base, output_format_content),
            }
        }

        match &options.prefix {
            RenderSetting::Always(prefix) => Some(prefix.as_str()),
            RenderSetting::Never => None,
            RenderSetting::Auto => auto_prefix(&self.target, self),
        }
    }

    fn enum_to_string(&self, enm: &Enum, options: &RenderOptions) -> String {
        EnumRender {
            name: enm.name.rendered_name().to_string(),
            delimiter: "----".into(),
            values: enm
                .values
                .iter()
                .map(|(name, description)| Attribute {
                    name: name.rendered_name().to_string(),
                    description: description.clone(),
                })
                .collect(),
        }
        .to_string(options)
    }

    fn inner_type_render(
        &self,
        options: &RenderOptions,
        field: &FieldType,
        render_state: &mut RenderState,
        group_hoisted_literals: bool,
    ) -> Result<String, minijinja::Error> {
        Ok(match field {
            FieldType::Primitive(t) => match t {
                TypeValue::String => "string".to_string(),
                TypeValue::Int => "int".to_string(),
                TypeValue::Float => "float".to_string(),
                TypeValue::Bool => "bool".to_string(),
                TypeValue::Null => "null".to_string(),
                TypeValue::Media(media_type) => {
                    return Err(minijinja::Error::new(
                        minijinja::ErrorKind::BadSerialization,
                        format!("type '{media_type}' is not supported in outputs"),
                    ))
                }
            },
            FieldType::Literal(v) => v.to_string(),
            FieldType::Constrained { base, .. } => {
                self.inner_type_render(options, base, render_state, group_hoisted_literals)?
            }
            FieldType::Enum(e) => {
                let Some(enm) = self.enums.get(e) else {
                    return Err(minijinja::Error::new(
                        minijinja::ErrorKind::BadSerialization,
                        format!("Enum {e} not found"),
                    ));
                };

                if enm.values.len() <= 6
                    && enm.values.iter().all(|(_, d)| d.is_none())
                    && !group_hoisted_literals
                    && !matches!(options.always_hoist_enums, RenderSetting::Always(true))
                {
                    let values = enm
                        .values
                        .iter()
                        .map(|(n, _)| format!("'{}'", n.rendered_name()))
                        .collect::<Vec<_>>()
                        .join(&options.or_splitter);

                    values
                } else {
                    render_state.hoisted_enums.insert(enm.name.name.clone());
                    enm.name.rendered_name().to_string()
                }
            }
            FieldType::Class(cls) => {
                let Some(class) = self.classes.get(cls) else {
                    return Err(minijinja::Error::new(
                        minijinja::ErrorKind::BadSerialization,
                        format!("Class {cls} not found"),
                    ));
                };

                // Hoist recursive classes.
                //
                // TODO: Some cloning in this function again, check
                // baml-lib/jsonish/src/tests/mod.rs
                // there's room for optimization.
                if render_state.hoisted_classes.len() < self.recursive_classes.len() {
                    for recursive_class in self.recursive_classes.iter() {
                        render_state.hoisted_classes.insert(recursive_class.clone());
                    }
                }

                ClassRender {
                    name: class.name.rendered_name().to_string(),
                    values: class
                        .fields
                        .iter()
                        .map(|(name, field_type, description)| {
                            let mut maybe_nested_recursive_class = None;
                            let mut is_optional = false;
                            let mut is_list = false;

                            match field_type {
                                // Non-optional class, part of a cycle.
                                FieldType::Class(nested_class)
                                    if self.recursive_classes.contains(nested_class) =>
                                {
                                    maybe_nested_recursive_class = Some(nested_class);
                                }

                                // Optional class, part of a cycle.
                                FieldType::Optional(boxed_field_type) => {
                                    if let FieldType::Class(nested_class) =
                                        boxed_field_type.as_ref()
                                    {
                                        if self.recursive_classes.contains(nested_class) {
                                            maybe_nested_recursive_class = Some(nested_class);
                                            is_optional = true;
                                        }
                                    }
                                }

                                // List class, part of a cycle.
                                FieldType::List(boxed_field_type) => {
                                    if let FieldType::Class(nested_class) =
                                        boxed_field_type.as_ref()
                                    {
                                        if self.recursive_classes.contains(nested_class) {
                                            maybe_nested_recursive_class = Some(nested_class);
                                            is_list = true;
                                        }
                                    }
                                }
                                _ => {}
                            }

                            // Terminate recursion. There's no other way to
                            // refer to a recursive class other than by name,
                            // and all recursive classes are hoisted so they'll
                            // be handled at a later stage.
                            let r#type = if let Some(nested_class) = maybe_nested_recursive_class {
                                if is_optional {
                                    format!("{nested_class}{}null", options.or_splitter)
                                } else if is_list {
                                    format!("{nested_class}[]")
                                } else {
                                    nested_class.to_string()
                                }
                            } else {
                                self.inner_type_render(options, field_type, render_state, false)?
                            };

                            Ok(ClassFieldRender {
                                name: name.rendered_name().to_string(),
                                description: description.clone(),
                                r#type,
                            })
                        })
                        .collect::<Result<_, minijinja::Error>>()?,
                }
                .to_string()
            }
            FieldType::List(inner) => {
                let inner_str = self.inner_type_render(options, inner, render_state, false)?;

                if match inner.as_ref() {
                    FieldType::Primitive(_) => false,
                    FieldType::Optional(t) => !t.is_primitive(),
                    FieldType::Enum(e) => inner_str.len() > 15,
                    _ => true,
                } {
                    format!("[\n  {}\n]", inner_str.replace('\n', "\n  "))
                } else {
                    if matches!(inner.as_ref(), FieldType::Optional(_)) {
                        format!("({})[]", inner_str)
                    } else {
                        format!("{}[]", inner_str)
                    }
                }
            }
            FieldType::Union(items) => items
                .iter()
                .map(|t| self.inner_type_render(options, t, render_state, true))
                .collect::<Result<Vec<_>, minijinja::Error>>()?
                .join(&options.or_splitter),
            FieldType::Optional(inner) => {
                let inner_str = self.inner_type_render(options, inner, render_state, false)?;
                if inner.is_optional() {
                    inner_str
                } else {
                    format!("{inner_str}{}null", options.or_splitter)
                }
            }
            FieldType::Tuple(_) => {
                return Err(minijinja::Error::new(
                    minijinja::ErrorKind::BadSerialization,
                    "Tuple type is not supported in outputs",
                ))
            }
            FieldType::Map(key_type, value_type) => MapRender {
                style: &options.map_style,
                key_type: self.inner_type_render(options, key_type, render_state, false)?,
                value_type: self.inner_type_render(options, value_type, render_state, false)?,
            }
            .to_string(),
        })
    }

    pub(crate) fn render(
        &self,
        options: RenderOptions,
    ) -> Result<Option<String>, minijinja::Error> {
        let prefix = self.prefix(&options);

        let mut render_state = RenderState {
            hoisted_enums: IndexSet::new(),
            hoisted_classes: IndexSet::new(),
        };

        let mut message = match &self.target {
            FieldType::Primitive(TypeValue::String) if prefix.is_none() => None,
            FieldType::Enum(e) => {
                let Some(enm) = self.enums.get(e) else {
                    return Err(minijinja::Error::new(
                        minijinja::ErrorKind::BadSerialization,
                        format!("Enum {} not found", e),
                    ));
                };

                Some(self.enum_to_string(enm, &options))
            }
            _ => Some(self.inner_type_render(&options, &self.target, &mut render_state, false)?),
        };

        // Top level recursive classes will just use their name instead of the
        // entire schema which should already be hoisted.
        if let FieldType::Class(class) = &self.target {
            if self.recursive_classes.contains(class) {
                message = Some(class.to_owned());
            }
        }

        let enum_definitions = render_state
            .hoisted_enums
            .iter()
            .map(|e| {
                let enm = self.enums.get(e).expect("Enum not found");
                self.enum_to_string(enm, &options)
            })
            .collect::<Vec<_>>();

        // Yeah we love the borrow checker...
        let hoisted_classes = std::mem::replace(&mut render_state.hoisted_classes, IndexSet::new());

        let mut class_definitions = Vec::new();

        for class_name in hoisted_classes {
            let schema = self.inner_type_render(
                &options,
                &FieldType::Class(class_name.to_owned()),
                &mut render_state,
                false,
            )?;

            // TODO: Prefix (type, interface, class, etc...) grab from &options.
            class_definitions.push(format!("{class_name} {schema}"));
        }

        let mut output = String::new();

        if enum_definitions.len() > 0 {
            output.push_str(&enum_definitions.join("\n\n"));
            output.push_str("\n\n");
        }

        if class_definitions.len() > 0 {
            output.push_str(&class_definitions.join("\n\n"));
            output.push_str("\n\n");
        }

        if let Some(p) = prefix {
            output.push_str(p);
        }

        if let Some(m) = message {
            output.push_str(&m);
        }

        // Trim end.
        while let Some('\n') = output.chars().last() {
            output.pop();
        }

        if output.is_empty() {
            Ok(None)
        } else {
            Ok(Some(output))
        }
    }
}

#[cfg(test)]
impl OutputFormatContent {
    pub fn new_array() -> Self {
        Self::target(FieldType::List(Box::new(FieldType::Primitive(
            TypeValue::String,
        ))))
        .build()
    }

    pub fn new_string() -> Self {
        Self::target(FieldType::Primitive(TypeValue::String)).build()
    }
}

impl OutputFormatContent {
    pub fn find_enum(&self, name: &str) -> Result<&Enum> {
        self.enums
            .get(name)
            .ok_or_else(|| anyhow::anyhow!("Enum {} not found", name))
    }

    pub fn find_class(&self, name: &str) -> Result<&Class> {
        self.classes
            .get(name)
            .ok_or_else(|| anyhow::anyhow!("Class {} not found", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_string() {
        let content = OutputFormatContent::new_string();
        let rendered = content.render(RenderOptions::default()).unwrap();
        assert_eq!(rendered, None);
    }

    #[test]
    fn test_render_array() {
        let content = OutputFormatContent::new_array();
        let rendered = content.render(RenderOptions::default()).unwrap();
        assert_eq!(
            rendered,
            Some("Answer with a JSON Array using this schema:\nstring[]".to_string())
        );
    }

    #[test]
    fn test_render_enum() {
        let enums = vec![Enum {
            name: Name::new("Color".to_string()),
            values: vec![
                (Name::new("Red".to_string()), None),
                (Name::new("Green".to_string()), None),
                (Name::new("Blue".to_string()), None),
            ],
            constraints: Vec::new(),
        }];

        let content = OutputFormatContent::target(FieldType::Enum("Color".to_string()))
            .enums(enums)
            .build();
        let rendered = content.render(RenderOptions::default()).unwrap();
        assert_eq!(
            rendered,
            Some(String::from(
                "Answer with any of the categories:\nColor\n----\n- Red\n- Green\n- Blue"
            ))
        );
    }

    #[test]
    fn test_render_class() {
        let classes = vec![Class {
            name: Name::new("Person".to_string()),
            fields: vec![
                (
                    Name::new("name".to_string()),
                    FieldType::Primitive(TypeValue::String),
                    Some("The person's name".to_string()),
                ),
                (
                    Name::new("age".to_string()),
                    FieldType::Primitive(TypeValue::Int),
                    Some("The person's age".to_string()),
                ),
            ],
            constraints: Vec::new(),
        }];

        let content = OutputFormatContent::target(FieldType::Class("Person".to_string()))
            .classes(classes)
            .build();
        let rendered = content.render(RenderOptions::default()).unwrap();
        assert_eq!(
            rendered,
            Some(String::from(
                "Answer in JSON using this schema:\n{\n  // The person's name\n  name: string,\n  // The person's age\n  age: int,\n}"
            ))
        );
    }

    #[test]
    fn test_render_class_with_multiline_descriptions() {
        let classes = vec![Class {
            name: Name::new("Education".to_string()),
            fields: vec![
                (
                    Name::new("school".to_string()),
                    FieldType::Optional(Box::new(FieldType::Primitive(TypeValue::String))),
                    Some("111\n  ".to_string()),
                ),
                (
                    Name::new("degree".to_string()),
                    FieldType::Primitive(TypeValue::String),
                    Some("2222222".to_string()),
                ),
                (
                    Name::new("year".to_string()),
                    FieldType::Primitive(TypeValue::Int),
                    None,
                ),
            ],
            constraints: Vec::new(),
        }];

        let content = OutputFormatContent::target(FieldType::Class("Education".to_string()))
            .classes(classes)
            .build();
        let rendered = content.render(RenderOptions::default()).unwrap();
        assert_eq!(
            rendered,
            Some(String::from(
                "Answer in JSON using this schema:\n{\n  // 111\n  //   \n  school: string or null,\n  // 2222222\n  degree: string,\n  year: int,\n}"
            ))
        );
    }

    #[test]
    fn test_render_top_level_simple_recursive_class() {
        let classes = vec![Class {
            name: Name::new("Node".to_string()),
            fields: vec![
                (
                    Name::new("data".to_string()),
                    FieldType::Primitive(TypeValue::Int),
                    None,
                ),
                (
                    Name::new("next".to_string()),
                    FieldType::Optional(Box::new(FieldType::Class("Node".to_string()))),
                    None,
                ),
            ],
            constraints: Vec::new(),
        }];

        let content = OutputFormatContent::target(FieldType::Class("Node".to_string()))
            .classes(classes)
            .recursive_classes(IndexSet::from_iter(["Node".to_string()]))
            .build();
        let rendered = content.render(RenderOptions::default()).unwrap();
        #[rustfmt::skip]
        assert_eq!(
            rendered,
            Some(String::from(
r#"Node {
  data: int,
  next: Node or null,
}

Answer in JSON using this schema: Node"#
            ))
        );
    }

    #[test]
    fn test_render_nested_simple_recursive_class() {
        let classes = vec![
            Class {
                name: Name::new("Node".to_string()),
                fields: vec![
                    (Name::new("data".to_string()), FieldType::int(), None),
                    (
                        Name::new("next".to_string()),
                        FieldType::Optional(Box::new(FieldType::Class("Node".to_string()))),
                        None,
                    ),
                ],
                constraints: Vec::new(),
            },
            Class {
                name: Name::new("LinkedList".to_string()),
                fields: vec![
                    (
                        Name::new("head".to_string()),
                        FieldType::Optional(Box::new(FieldType::Class("Node".to_string()))),
                        None,
                    ),
                    (Name::new("len".to_string()), FieldType::int(), None),
                ],
                constraints: Vec::new(),
            },
        ];

        let content = OutputFormatContent::target(FieldType::Class("LinkedList".to_string()))
            .classes(classes)
            .recursive_classes(IndexSet::from_iter(["Node".to_string()]))
            .build();
        let rendered = content.render(RenderOptions::default()).unwrap();
        #[rustfmt::skip]
        assert_eq!(
            rendered,
            Some(String::from(
r#"Node {
  data: int,
  next: Node or null,
}

Answer in JSON using this schema:
{
  head: Node or null,
  len: int,
}"#
            ))
        );
    }

    #[test]
    fn test_top_level_recursive_cycle() {
        let classes = vec![
            Class {
                name: Name::new("A".to_string()),
                fields: vec![(
                    Name::new("pointer".to_string()),
                    FieldType::Class("B".to_string()),
                    None,
                )],
                constraints: Vec::new(),
            },
            Class {
                name: Name::new("B".to_string()),
                fields: vec![(
                    Name::new("pointer".to_string()),
                    FieldType::Class("C".to_string()),
                    None,
                )],
                constraints: Vec::new(),
            },
            Class {
                name: Name::new("C".to_string()),
                fields: vec![(
                    Name::new("pointer".to_string()),
                    FieldType::Optional(Box::new(FieldType::Class("A".to_string()))),
                    None,
                )],
                constraints: Vec::new(),
            },
        ];

        let content = OutputFormatContent::target(FieldType::Class("A".to_string()))
            .classes(classes)
            .recursive_classes(IndexSet::from_iter(
                ["A", "B", "C"].map(ToString::to_string),
            ))
            .build();
        let rendered = content.render(RenderOptions::default()).unwrap();
        #[rustfmt::skip]
        assert_eq!(
            rendered,
            Some(String::from(
r#"A {
  pointer: B,
}

B {
  pointer: C,
}

C {
  pointer: A or null,
}

Answer in JSON using this schema: A"#
            ))
        );
    }

    #[test]
    fn test_nested_recursive_cycle() {
        let classes = vec![
            Class {
                name: Name::new("A".to_string()),
                fields: vec![(
                    Name::new("pointer".to_string()),
                    FieldType::Class("B".to_string()),
                    None,
                )],
                constraints: Vec::new(),
            },
            Class {
                name: Name::new("B".to_string()),
                fields: vec![(
                    Name::new("pointer".to_string()),
                    FieldType::Class("C".to_string()),
                    None,
                )],
                constraints: Vec::new(),
            },
            Class {
                name: Name::new("C".to_string()),
                fields: vec![(
                    Name::new("pointer".to_string()),
                    FieldType::Optional(Box::new(FieldType::Class("A".to_string()))),
                    None,
                )],
                constraints: Vec::new(),
            },
            Class {
                name: Name::new("NonRecursive".to_string()),
                fields: vec![
                    (
                        Name::new("pointer".to_string()),
                        FieldType::Class("A".to_string()),
                        None,
                    ),
                    (Name::new("data".to_string()), FieldType::int(), None),
                    (Name::new("field".to_string()), FieldType::bool(), None),
                ],
                constraints: Vec::new(),
            },
        ];

        let content = OutputFormatContent::target(FieldType::Class("NonRecursive".to_string()))
            .classes(classes)
            .recursive_classes(IndexSet::from_iter(
                ["A", "B", "C"].map(ToString::to_string),
            ))
            .build();
        let rendered = content.render(RenderOptions::default()).unwrap();
        #[rustfmt::skip]
        assert_eq!(
            rendered,
            Some(String::from(
r#"A {
  pointer: B,
}

B {
  pointer: C,
}

C {
  pointer: A or null,
}

Answer in JSON using this schema:
{
  pointer: A,
  data: int,
  field: bool,
}"#
            ))
        );
    }

    #[test]
    fn test_nested_class_in_hoisted_recursive_class() {
        let classes = vec![
            Class {
                name: Name::new("A".to_string()),
                fields: vec![
                    (
                        Name::new("pointer".to_string()),
                        FieldType::Class("B".to_string()),
                        None,
                    ),
                    (
                        Name::new("nested".to_string()),
                        FieldType::Class("Nested".to_string()),
                        None,
                    ),
                ],
                constraints: Vec::new(),
            },
            Class {
                name: Name::new("B".to_string()),
                fields: vec![(
                    Name::new("pointer".to_string()),
                    FieldType::Class("C".to_string()),
                    None,
                )],
                constraints: Vec::new(),
            },
            Class {
                name: Name::new("C".to_string()),
                fields: vec![(
                    Name::new("pointer".to_string()),
                    FieldType::Optional(Box::new(FieldType::Class("A".to_string()))),
                    None,
                )],
                constraints: Vec::new(),
            },
            Class {
                name: Name::new("NonRecursive".to_string()),
                fields: vec![
                    (
                        Name::new("pointer".to_string()),
                        FieldType::Class("A".to_string()),
                        None,
                    ),
                    (Name::new("data".to_string()), FieldType::int(), None),
                    (Name::new("field".to_string()), FieldType::bool(), None),
                ],
                constraints: Vec::new(),
            },
            Class {
                name: Name::new("Nested".to_string()),
                fields: vec![
                    (Name::new("data".to_string()), FieldType::int(), None),
                    (Name::new("field".to_string()), FieldType::bool(), None),
                ],
                constraints: Vec::new(),
            },
        ];

        let content = OutputFormatContent::target(FieldType::Class("NonRecursive".to_string()))
            .classes(classes)
            .recursive_classes(IndexSet::from_iter(
                ["A", "B", "C"].map(ToString::to_string),
            ))
            .build();
        let rendered = content.render(RenderOptions::default()).unwrap();
        #[rustfmt::skip]
            assert_eq!(
                rendered,
                Some(String::from(
r#"A {
  pointer: B,
  nested: {
    data: int,
    field: bool,
  },
}

B {
  pointer: C,
}

C {
  pointer: A or null,
}

Answer in JSON using this schema:
{
  pointer: A,
  data: int,
  field: bool,
}"#
            ))
        );
    }

    #[test]
    fn test_mutually_recursive_list() {
        let classes = vec![
            Class {
                name: Name::new("Tree".to_string()),
                fields: vec![
                    (Name::new("data".to_string()), FieldType::int(), None),
                    (
                        Name::new("children".to_string()),
                        FieldType::Class("Forest".to_string()),
                        None,
                    ),
                ],
                constraints: Vec::new(),
            },
            Class {
                name: Name::new("Forest".to_string()),
                fields: vec![(
                    Name::new("trees".to_string()),
                    FieldType::List(Box::new(FieldType::Class("Tree".to_string()))),
                    None,
                )],
                constraints: Vec::new(),
            },
        ];

        let content = OutputFormatContent::target(FieldType::Class("Tree".to_string()))
            .classes(classes)
            .recursive_classes(IndexSet::from_iter(
                ["Tree", "Forest"].map(ToString::to_string),
            ))
            .build();
        let rendered = content.render(RenderOptions::default()).unwrap();
        #[rustfmt::skip]
            assert_eq!(
                rendered,
                Some(String::from(
r#"Tree {
  data: int,
  children: Forest,
}

Forest {
  trees: Tree[],
}

Answer in JSON using this schema: Tree"#
            ))
        );
    }
}
