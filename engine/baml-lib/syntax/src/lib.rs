mod forms;
use lalrpop_util::lalrpop_mod;
pub mod pos;

lalrpop_mod!(pub grammar);

#[cfg(test)]
mod tests {
    use internal_baml_diagnostics::{Diagnostics, SourceFile};

    use super::*;
    use crate::forms::r#type::{Type, BuiltinType};
    use crate::grammar::*;

    #[test]
    fn test_parse_identifier() {
        let source_file = SourceFile::new_static("tmp.baml".into(), "");
        let mut diagnostics = Diagnostics::new("tmp.baml".into());
        let identifier = IdentifierParser::new().parse(&source_file, &mut diagnostics, "foo").unwrap();
        assert_eq!(identifier.name, "foo");
    }

    #[test]
    fn test_parse_class() {
        let source_file = SourceFile::new_static("tmp.baml".into(), "");
        let mut diagnostics = Diagnostics::new("tmp.baml".into());
        let class = ClassParser::new().parse(&source_file, &mut diagnostics, r#"
          class Foo {
            bar int
            baz string
          }
        "#).unwrap();
        assert_eq!(class.name.to_string().as_str(), "Foo");
        match class.fields.as_slice() {
            [field1, field2] => {
                assert_eq!(field1.name.to_string().as_str(), "bar");
                assert!(matches!(field1.r#type, Type::Builtin{ builtin_type: BuiltinType::Int, ..} ));
                assert_eq!(field2.name.to_string().as_str(), "baz");
                dbg!(&field2);
                assert!(matches!(field2.r#type, Type::Builtin{ builtin_type: BuiltinType::String, ..} ));
            },
            _ => {panic!("Expected 2 fields");}
        }
    }

}

// use internal_baml_diagnostics::{DatamodelError, DatamodelWarning, Diagnostics, Span};
// use crate::lexer::BAMLParser;
//  use crate::forms::{Function, Enum};
// 
// /// The abstract syntax of the BAML language, paired with an arbitrary
// /// type of metadata. Metadata will typically be a `Span`.
// pub struct Ast<T> {
//     pub top_levels: Vec<AstTopLevel<T>>,
//     pub meta: T
// }
// 
// /// The different elements that can be found an the top level of a BAML
// /// source file.
// pub enum AstTopLevel<T> {
//     // Class(Class<T>),
//     Enum (Enum<T>),
//     Function (Function<T>),
//     // TemplateString (TemplateString<T>),
//     // Test (Test<T>)
// }
// 
// /// Parsing at the top level always returns an `Ast`. It is
// /// infallible so that any errors encountered during parsing
// /// must be handled as diagnostics.
// /// 
// /// When encountering some kind of error during parsing,
// /// we prefer to add an error to the `Diagnostics` context
// /// and return a partial AST element.
// pub fn parse(_diagnostics: &mut Diagnostics) -> Ast<Span> {
//     unimplemented!()
// }
