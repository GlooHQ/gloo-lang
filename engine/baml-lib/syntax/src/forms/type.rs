/// The syntax of a type.
///
/// For example:
/// ```baml
/// int                    // bare builtin type
/// string                 // bare builtin type
/// MyType                 // bare user type
/// 1                      // literal type
/// int | string           // choice type (union)
/// int[] | MyType?        // choice of list or optional
/// (int | bool[])[]       // list of (int or list of bool)
/// int @attr(arg) | bool  // int with attr or bool
/// ```
///
/// These types are modeled in the syntax
use internal_baml_diagnostics::Span;

use crate::forms::attribute::Attribute;
use crate::forms::identifier::Identifier;
use crate::pos::WithPos;

use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum Type<T> {
    Builtin {
        builtin_type: BuiltinType,
        meta: T,
    },
    UserDefined {
        name: Identifier<T>,
    },
    Option {
        base_type: Box<Type<T>>,
        meta: T,
    },
    List {
        base_type: Box<Type<T>>,
        meta: T,
    },
    Union {
        variants: Vec<Type<T>>,
        meta: T,
    },
    WithAttributes {
        base_type: Box<Type<T>>,
        attributes: Vec<Attribute<T>>,
        meta: T,
    },
    Error {
        meta: T,
    },
}

impl WithPos for Type<Span> {
    fn with_pos(self, pos: Span) -> Self {
        use Type::*;
        match self {
            Builtin { builtin_type, .. } => Builtin {
                builtin_type,
                meta: pos,
            },
            UserDefined { name } => UserDefined {
                name: name.with_pos(pos),
            },
            Option { base_type, .. } => Option {
                base_type,
                meta: pos,
            },
            List { base_type, .. } => List {
                base_type,
                meta: pos,
            },
            Union { variants, .. } => Union {
                variants,
                meta: pos,
            },
            WithAttributes {
                base_type,
                attributes,
                ..
            } => WithAttributes {
                base_type,
                attributes,
                meta: pos,
            },
            Error { .. } => Error { meta: pos },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BuiltinType {
    Int,
    String,
    Float,
    Bool,
    Image,
    Audio,
}

impl FromStr for BuiltinType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        use BuiltinType::*;
        match s {
            "int" => Ok(Int),
            "float" => Ok(Float),
            "bool" => Ok(Bool),
            "string" => Ok(String),
            "image" => Ok(Image),
            "audio" => Ok(Audio),
            _ => unreachable!("Ruled out by the parser"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::TypeParser;
    use internal_baml_diagnostics::{Diagnostics, SourceFile};

    #[test]
    fn test_parse_type() {
        let source_file = SourceFile::new_static("tmp.baml".into(), "");
        let p = TypeParser::new();

        let mut diagnostics = Diagnostics::new("tmp.baml".into());

        assert!(matches!(
            p.parse(&source_file, &mut diagnostics, "int").unwrap(),
            Type::Builtin {
                builtin_type: BuiltinType::Int,
                ..
            }
        ));

        let int_list = p.parse(&source_file, &mut diagnostics, "int[]").unwrap();
        match int_list {
            Type::List { base_type, .. } => {
                assert!(matches!(
                    *base_type,
                    Type::Builtin {
                        builtin_type: BuiltinType::Int,
                        ..
                    }
                ));
            }
            _ => {
                panic!("Expected list")
            }
        }

        let int_list_list = p.parse(&source_file, &mut diagnostics, "int[][]").unwrap();
        match int_list_list {
            Type::List { base_type, .. } => match *base_type {
                Type::List { base_type, .. } => {
                    assert!(matches!(
                        *base_type,
                        Type::Builtin {
                            builtin_type: BuiltinType::Int,
                            ..
                        }
                    ));
                }
                _ => {
                    panic!("Expected list")
                }
            },
            _ => {
                panic!("Expected list")
            }
        }

        let int_option_list_parens = p.parse(&source_file, &mut diagnostics, "(int?)[]").unwrap();
        match int_option_list_parens {
            Type::List { base_type, .. } => match *base_type {
                Type::Option { base_type, .. } => {
                    assert!(matches!(
                        *base_type,
                        Type::Builtin {
                            builtin_type: BuiltinType::Int,
                            ..
                        }
                    ));
                }
                _ => {
                    panic!("Expected list")
                }
            },
            _ => {
                panic!("Expected list")
            }
        }

        let int_option_option = p.parse(&source_file, &mut diagnostics, "int??").unwrap();
        dbg!(&diagnostics);
        dbg!(&int_option_option);
        assert!(matches!(int_option_option, Type::Error { .. }));

        let simple_union = p
            .parse(&source_file, &mut diagnostics, "int | bool")
            .unwrap();
        match simple_union {
            Type::Union { variants, .. } => match variants.as_slice() {
                [variant1, variant2] => {
                    assert!(matches!(
                        variant1,
                        Type::Builtin {
                            builtin_type: BuiltinType::Int,
                            ..
                        }
                    ));
                    assert!(matches!(
                        variant2,
                        Type::Builtin {
                            builtin_type: BuiltinType::Bool,
                            ..
                        }
                    ));
                }
                _ => {
                    panic!("Expected 2 variants");
                }
            },
            _ => {
                panic!("Expected union");
            }
        }
    }
}
