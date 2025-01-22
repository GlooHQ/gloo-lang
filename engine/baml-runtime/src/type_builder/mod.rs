use std::sync::{Arc, Mutex};
use std::fmt;

use baml_types::{BamlValue, FieldType};
use indexmap::IndexMap;

use crate::runtime_context::{PropertyAttributes, RuntimeClassOverride, RuntimeEnumOverride};

type MetaData = Arc<Mutex<IndexMap<String, BamlValue>>>;

trait Meta {
    fn meta(&self) -> MetaData;
}

pub trait WithMeta {
    fn with_meta(&self, key: &str, value: BamlValue) -> &Self;
}

macro_rules! impl_meta {
    ($type:ty) => {
        impl Meta for $type {
            fn meta(&self) -> MetaData {
                self.meta.clone()
            }
        }
    };
}

impl<T> WithMeta for T
where
    T: Meta,
{
    fn with_meta(&self, key: &str, value: BamlValue) -> &T {
        let meta = self.meta();
        let mut meta = meta.lock().unwrap();
        meta.insert(key.to_string(), value);
        self
    }
}

impl<T: Meta> From<&Arc<Mutex<T>>> for PropertyAttributes {
    fn from(value: &Arc<Mutex<T>>) -> Self {
        let value = value.lock().unwrap();
        let meta = value.meta();
        let meta = meta.lock().unwrap();
        let properties = meta.clone();
        let alias = properties.get("alias").cloned();
        let skip = properties.get("skip").and_then(|v| v.as_bool());

        Self {
            alias,
            skip,
            meta: properties,
        }
    }
}

pub struct ClassBuilder {
    properties: Arc<Mutex<IndexMap<String, Arc<Mutex<ClassPropertyBuilder>>>>>,
    meta: MetaData,
}
impl_meta!(ClassBuilder);

pub struct ClassPropertyBuilder {
    r#type: Arc<Mutex<Option<FieldType>>>,
    meta: MetaData,
}
impl_meta!(ClassPropertyBuilder);

impl ClassPropertyBuilder {
    pub fn r#type(&self, r#type: FieldType) -> &Self {
        *self.r#type.lock().unwrap() = Some(r#type);
        self
    }
}

impl Default for ClassBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ClassBuilder {
    pub fn new() -> Self {
        Self {
            properties: Default::default(),
            meta: Arc::new(Mutex::new(Default::default())),
        }
    }

    pub fn property(&self, name: &str) -> Arc<Mutex<ClassPropertyBuilder>> {
        let mut properties = self.properties.lock().unwrap();
        Arc::clone(properties.entry(name.to_string()).or_insert_with(|| {
            Arc::new(Mutex::new(ClassPropertyBuilder {
                r#type: Default::default(),
                meta: Default::default(),
            }))
        }))
    }
}

pub struct EnumBuilder {
    values: Arc<Mutex<IndexMap<String, Arc<Mutex<EnumValueBuilder>>>>>,
    meta: MetaData,
}
impl_meta!(EnumBuilder);

pub struct EnumValueBuilder {
    meta: MetaData,
}
impl_meta!(EnumValueBuilder);

impl Default for EnumBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl EnumBuilder {
    pub fn new() -> Self {
        Self {
            values: Default::default(),
            meta: Arc::new(Mutex::new(Default::default())),
        }
    }

    pub fn value(&self, name: &str) -> Arc<Mutex<EnumValueBuilder>> {
        let mut values = self.values.lock().unwrap();
        Arc::clone(values.entry(name.to_string()).or_insert_with(|| {
            Arc::new(Mutex::new(EnumValueBuilder {
                meta: Default::default(),
            }))
        }))
    }
}

// displays a class property along with its current state and metadata
// the format shows three key pieces of information:
// 1. the property name as defined in the class
// 2. the type status: either 'set' (type defined) or 'unset' (type pending)
// 3. any metadata attached to the property in parentheses
//
// metadata is shown in key=value format, with values formatted according to their type
// multiple metadata entries are separated by commas for readability
//
// examples of the output format:
//   name set (alias='username', description='full name')
//   - shows a property with both alias and description metadata
//   age unset
//   - shows a property without a defined type or metadata
//   email set (required=true, format='email')
//   - shows a property with multiple metadata values of different types
impl fmt::Display for ClassPropertyBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let meta = self.meta.lock().unwrap();
        write!(f, "{}", self.r#type.lock().unwrap().as_ref().map_or("unset", |_| "set"))?;

        if !meta.is_empty() {
            write!(f, " (")?;
            for (i, (key, value)) in meta.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}={}", key, value)?;
            }
            write!(f, ")")?;
        }
        Ok(())
    }
}

// displays an enum value and its associated metadata
// the format focuses on clarity and completeness:
// 1. the enum value name in uppercase (following enum conventions)
// 2. any metadata in parentheses, showing all attached information
//
// metadata is displayed in a consistent key=value format:
// - each piece of metadata is separated by commas
// - values are formatted based on their type (quotes for strings, etc.)
// - all metadata is shown, not just common fields like alias
//
// examples of the output format:
//   ACTIVE (alias='active', priority=1, enabled=true)
//   - shows an enum value with multiple metadata types
//   PENDING
//   - shows a simple enum value with no metadata
//   INACTIVE (description='not currently in use', status=null)
//   - shows how null values and longer descriptions are formatted
impl fmt::Display for EnumValueBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let meta = self.meta.lock().unwrap();

        if !meta.is_empty() {
            write!(f, " (")?;
            for (i, (key, value)) in meta.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}={}", key, value)?;
            }
            write!(f, ")")?;
        }
        Ok(())
    }
}

// displays a complete class definition with all its properties
// the format provides a clear hierarchical structure:
// 1. class name followed by an opening brace
// 2. indented list of properties, each on its own line
// 3. closing brace aligned with the class name
//
// properties are displayed with consistent indentation and formatting:
// - each property starts on a new line with proper indentation
// - properties are separated by commas for valid syntax
// - the last property doesn't have a trailing comma
//
// example of the complete format:
//   User {
//     name set (alias='username', description='user's full name'),
//     age set (type='integer', min=0),
//     email set (format='email', required=true),
//     status unset
//   }
impl fmt::Display for ClassBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let properties = self.properties.lock().unwrap();
        write!(f, "{{")?;
        if !properties.is_empty() {
            for (i, (name, prop)) in properties.iter().enumerate() {
                if i > 0 {
                    write!(f, ",")?;
                }
                write!(f, "\n      {} {}", name, prop.lock().unwrap())?;
            }
            write!(f, "\n    ")?;
        }
        write!(f, "}}")
    }
}

// displays a complete enum definition with all its values
// the format creates a clear and readable structure:
// 1. enum name followed by an opening brace
// 2. indented list of enum values, each on its own line
// 3. closing brace aligned with the enum name
//
// values are displayed with consistent formatting:
// - each value starts on a new line with proper indentation
// - values are separated by commas for valid syntax
// - metadata is shown in parentheses when present
// - empty enums are shown with empty braces
//
// example of the complete format:
//   Status {
//     ACTIVE (alias='active', weight=1.0),
//     PENDING (description='awaiting processing'),
//     INACTIVE (enabled=false),
//     ARCHIVED
//   }
impl fmt::Display for EnumBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let values = self.values.lock().unwrap();
        write!(f, "{{")?;
        if !values.is_empty() {
            for (i, (name, value)) in values.iter().enumerate() {
                if i > 0 {
                    write!(f, ",")?;
                }
                write!(f, "\n      {}{}", name, value.lock().unwrap())?;
            }
            write!(f, "\n    ")?;
        }
        write!(f, "}}")
    }
}

// displays the complete type builder state in a clear, hierarchical format
// this is the top-level representation that shows all defined types
//
//
// 1. starts with "TypeBuilder(" to identify the structure
// 2. contains two main sections: Classes and Enums
// 3. each section is properly indented and bracketed
// 4. empty sections are omitted for conciseness
//
// the structure maintains consistent formatting:
// - each class and enum starts on a new line
// - proper indentation shows the hierarchy
// - commas separate multiple items
// - empty classes/enums are shown with empty braces
//
// example of the complete format:
// TypeBuilder(
//   Classes: [
//     User {
//       name set (alias='username'),
//       email set (required=true)
//     },
//     Address { }
//   ],
//   Enums: [
//     Status {
//       ACTIVE (alias='active'),
//       PENDING,
//       INACTIVE (enabled=false)
//     }
//   ]
// )
//
// this format makes it easy to:
// - understand the overall structure of defined types
// - see relationships between classes and their properties
// - identify enum values and their metadata
// - spot any missing or incomplete definitions
impl fmt::Display for TypeBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let classes = self.classes.lock().unwrap();
        let enums = self.enums.lock().unwrap();

        writeln!(f, "TypeBuilder(")?;

        if !classes.is_empty() {
            write!(f, "  Classes: [")?;
            for (i, (name, cls)) in classes.iter().enumerate() {
                if i > 0 {
                    write!(f, ",")?;
                }
                write!(f, "\n    {} {}", name, cls.lock().unwrap())?;
            }
            write!(f, "\n  ]")?;
        }

        if !enums.is_empty() {
            if !classes.is_empty() {
                write!(f, ",")?;
            }
            write!(f, "\n  Enums: [")?;
            for (i, (name, e)) in enums.iter().enumerate() {
                if i > 0 {
                    write!(f, ",")?;
                }
                write!(f, "\n    {} {}", name, e.lock().unwrap())?;
            }
            write!(f, "\n  ]")?;
        }

        write!(f, "\n)")
    }
}

#[derive(Clone)]
pub struct TypeBuilder {
    classes: Arc<Mutex<IndexMap<String, Arc<Mutex<ClassBuilder>>>>>,
    enums: Arc<Mutex<IndexMap<String, Arc<Mutex<EnumBuilder>>>>>,
}

impl Default for TypeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeBuilder {
    pub fn new() -> Self {
        Self {
            classes: Default::default(),
            enums: Default::default(),
        }
    }

    pub fn class(&self, name: &str) -> Arc<Mutex<ClassBuilder>> {
        Arc::clone(
            self.classes
                .lock()
                .unwrap()
                .entry(name.to_string())
                .or_insert_with(|| Arc::new(Mutex::new(ClassBuilder::new()))),
        )
    }

    pub fn r#enum(&self, name: &str) -> Arc<Mutex<EnumBuilder>> {
        Arc::clone(
            self.enums
                .lock()
                .unwrap()
                .entry(name.to_string())
                .or_insert_with(|| Arc::new(Mutex::new(EnumBuilder::new()))),
        )
    }

    pub fn to_overrides(
        &self,
    ) -> (
        IndexMap<String, RuntimeClassOverride>,
        IndexMap<String, RuntimeEnumOverride>,
    ) {
        log::debug!("Converting types to overrides");
        let cls = self
            .classes
            .lock()
            .unwrap()
            .iter()
            .map(|(name, cls)| {
                log::debug!("Converting class: {}", name);
                let mut overrides = RuntimeClassOverride {
                    alias: None,
                    new_fields: Default::default(),
                    update_fields: Default::default(),
                };

                cls.lock()
                    .unwrap()
                    .properties
                    .lock()
                    .unwrap()
                    .iter()
                    .for_each(|(property_name, f)| {
                        let attrs = PropertyAttributes::from(f);
                        let t = {
                            let property = f.lock().unwrap();
                            let t = property.r#type.lock().unwrap();
                            t.clone()
                        };
                        match t.as_ref() {
                            Some(r#type) => {
                                overrides
                                    .new_fields
                                    .insert(property_name.to_string(), (r#type.clone(), attrs));
                            }
                            None => {
                                overrides
                                    .update_fields
                                    .insert(property_name.to_string(), attrs);
                            }
                        }
                    });
                (name.clone(), overrides)
            })
            .collect();

        let enm = self
            .enums
            .lock()
            .unwrap()
            .iter()
            .map(|(name, enm)| {
                let attributes = PropertyAttributes::from(enm);
                let values = enm
                    .lock()
                    .unwrap()
                    .values
                    .lock()
                    .unwrap()
                    .iter()
                    .map(|(value_name, value)| {
                        (value_name.clone(), PropertyAttributes::from(value))
                    })
                    .collect();
                (
                    name.clone(),
                    RuntimeEnumOverride {
                        values,
                        alias: attributes.alias,
                    },
                )
            })
            .collect();
        log::debug!(
            "Dynamic types: \n {:#?} \n Dynamic enums\n {:#?} enums",
            cls,
            enm
        );
        (cls, enm)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_builder() {
        let builder = TypeBuilder::new();

        // Add a class with properties and metadata
        let cls = builder.class("User");
        {
            let cls = cls.lock().unwrap();
            // Add name property with alias and description
            cls.property("name")
                .lock()
                .unwrap()
                .r#type(FieldType::string())
                .with_meta("alias", BamlValue::String("username".to_string()))
                .with_meta("description", BamlValue::String("The user's full name".to_string()));

            // Add age property with description only
            cls.property("age")
                .lock()
                .unwrap()
                .r#type(FieldType::int())
                .with_meta("description", BamlValue::String("User's age in years".to_string()));

            // Add email property with no metadata
            cls.property("email")
                .lock()
                .unwrap()
                .r#type(FieldType::string());
        }

        // Add an enum with values and metadata
        let enm = builder.r#enum("Status");
        {
            let enm = enm.lock().unwrap();
            // Add ACTIVE value with alias and description
            enm.value("ACTIVE")
                .lock()
                .unwrap()
                .with_meta("alias", BamlValue::String("active".to_string()))
                .with_meta("description", BamlValue::String("User is active".to_string()));

            // Add INACTIVE value with alias only
            enm.value("INACTIVE")
                .lock()
                .unwrap()
                .with_meta("alias", BamlValue::String("inactive".to_string()));

            // Add PENDING value with no metadata
            enm.value("PENDING");
        }

        // Convert to string and verify the format
        let output = builder.to_string();
        assert_eq!(
            output,
            "TypeBuilder(\n  Classes: [\n    User {\n      name set (alias=String(\"username\"), description=String(\"The user's full name\")),\n      age set (description=String(\"User's age in years\")),\n      email set\n    }\n  ],\n  Enums: [\n    Status {\n      ACTIVE (alias=String(\"active\"), description=String(\"User is active\")),\n      INACTIVE (alias=String(\"inactive\")),\n      PENDING\n    }\n  ]\n)"
        );
    }

// my paranoia kicked in, so this  test is to ensure that the string representation is correct
// and that the to_overrides method is working as expected

    #[test]
    fn test_type_builder_advanced() {
        let builder = TypeBuilder::new();

        // 1. Complex class with nested types and all field types
        let address = builder.class("Address");
        {
            let address = address.lock().unwrap();
            // String with all metadata
            address.property("street")
                .lock()
                .unwrap()
                .r#type(FieldType::string())
                .with_meta("alias", BamlValue::String("streetAddress".to_string()))
                .with_meta("description", BamlValue::String("Street address including number".to_string()));

            // Optional int with description
            address.property("unit")
                .lock()
                .unwrap()
                .r#type(FieldType::int().as_optional())
                .with_meta("description", BamlValue::String("Apartment/unit number if applicable".to_string()));

            // List of strings with alias
            address.property("tags")
                .lock()
                .unwrap()
                .r#type(FieldType::string().as_list())
                .with_meta("alias", BamlValue::String("labels".to_string()));

            // Boolean with no metadata
            address.property("is_primary")
                .lock()
                .unwrap()
                .r#type(FieldType::bool());

            // Float with skip metadata
            address.property("coordinates")
                .lock()
                .unwrap()
                .r#type(FieldType::float())
                .with_meta("skip", BamlValue::Bool(true));
        }

        // 2. Empty class
        builder.class("EmptyClass");

        // 3. Complex enum with various metadata combinations
        let priority = builder.r#enum("Priority");
        {
            let priority = priority.lock().unwrap();
            // All metadata
            priority.value("HIGH")
                .lock()
                .unwrap()
                .with_meta("alias", BamlValue::String("urgent".to_string()))
                .with_meta("description", BamlValue::String("Needs immediate attention".to_string()))
                .with_meta("skip", BamlValue::Bool(false));

            // Only description
            priority.value("MEDIUM")
                .lock()
                .unwrap()
                .with_meta("description", BamlValue::String("Standard priority".to_string()));

            // Only skip
            priority.value("LOW")
                .lock()
                .unwrap()
                .with_meta("skip", BamlValue::Bool(true));

            // No metadata
            priority.value("NONE");
        }

        // 4. Empty enum
        builder.r#enum("EmptyEnum");

        // Test string representation
        let output = builder.to_string();
        assert_eq!(
            output,
             "TypeBuilder(\n  Classes: [\n    Address {\n      street set (alias=String(\"streetAddress\"), description=String(\"Street address including number\")),\n      unit set (description=String(\"Apartment/unit number if applicable\")),\n      tags set (alias=String(\"labels\")),\n      is_primary set,\n      coordinates set (skip=Bool(true))\n    },\n    EmptyClass {}\n  ],\n  Enums: [\n    Priority {\n      HIGH (alias=String(\"urgent\"), description=String(\"Needs immediate attention\"), skip=Bool(false)),\n      MEDIUM (description=String(\"Standard priority\")),\n      LOW (skip=Bool(true)),\n      NONE\n    },\n    EmptyEnum {}\n  ]\n)"
        );

        // Test to_overrides()
        let (classes, enums) = builder.to_overrides();

        // Verify class overrides
        assert_eq!(classes.len(), 2);
        let address_override = classes.get("Address").unwrap();
        assert_eq!(address_override.new_fields.len(), 5); // All fields are new
        assert!(address_override.new_fields.get("street").unwrap().1.alias.is_some());
        assert!(address_override.new_fields.get("coordinates").unwrap().1.skip.unwrap());

        // Verify enum overrides
        assert_eq!(enums.len(), 2);
        let priority_override = enums.get("Priority").unwrap();
        assert_eq!(priority_override.values.len(), 4);
        assert!(priority_override.values.get("HIGH").unwrap().alias.is_some());
        assert!(priority_override.values.get("LOW").unwrap().skip.unwrap());
    }
}
