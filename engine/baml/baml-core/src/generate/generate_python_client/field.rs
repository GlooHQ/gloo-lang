use internal_baml_parser_database::walkers::Walker;
use internal_baml_schema_ast::ast::{ClassId, FieldArity, FieldId, FieldType};

use super::{
    file::File,
    traits::{WithFile, WithPythonString},
};

impl WithPythonString for Walker<'_, (ClassId, FieldId)> {
    fn python_string(&self, file: &mut File) {
        file.add_indent_string(format!("{}: ", self.name()), 1);
        self.r#type().python_string(file);
        file.add_empty_line();
    }
}

impl WithPythonString for (FieldArity, &FieldType) {
    fn python_string(&self, file: &mut File) {
        match self.0 {
            FieldArity::Required => match self.1 {
                FieldType::PrimitiveType(s, _) => {
                    file.add_string(s);
                }
                FieldType::Supported(idn) => {
                    file.add_string(&idn.name);
                }
                FieldType::Union(types, _) => {
                    file.add_import("typing", "Union");
                    file.add_string("Union[");
                    for (i, (arity, field)) in types.iter().enumerate() {
                        if i > 0 {
                            file.add_string(", ");
                        }
                        (arity.clone(), field).python_string(file);
                    }
                    file.add_string("]");
                }
                FieldType::Unsupported(..) => panic!("Unsupported field type"),
            },
            FieldArity::List => {
                file.add_import("typing", "List");
                file.add_string("List[");
                (FieldArity::Required, self.1).python_string(file);
                file.add_string("]");
            }
            FieldArity::Optional => {
                file.add_import("typing", "Optional");
                file.add_string("Optional[");
                (FieldArity::Required, self.1).python_string(file);
                file.add_string("]");
            }
        }
    }
}
