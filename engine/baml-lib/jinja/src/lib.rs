mod evaluate_type;

use evaluate_type::get_variable_types;
pub use evaluate_type::{JinjaContext, PredefinedTypes, Type, TypeError};

#[derive(Debug)]
pub struct ValidationError {
    pub errors: Vec<TypeError>,
    pub parsing_errors: Option<minijinja::Error>,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for err in &self.errors {
            writeln!(f, "{err}")?;
        }
        Ok(())
    }
}

impl std::error::Error for ValidationError {}

pub fn validate_expression(
    expression: &str,
    types: &mut PredefinedTypes,
) -> Result<(), ValidationError> {
    let parsed = match minijinja::machinery::parse_expr(expression) {
        Ok(parsed) => parsed,
        Err(err) => {
            return Err(ValidationError {
                errors: vec![],
                parsing_errors: Some(err),
            });
        }
    };

    let expr_type = evaluate_type::evaluate_type(&parsed, types);
    match expr_type {
        Ok(_) => Ok(()),
        Err(err) => Err(ValidationError {
            errors: err,
            parsing_errors: None,
        }),
    }
}

pub fn validate_template(
    name: &str,
    template: &str,
    types: &mut PredefinedTypes,
) -> Result<(), ValidationError> {
    let parsed =
        match minijinja::machinery::parse(template, name, Default::default(), Default::default()) {
            Ok(parsed) => parsed,
            Err(err) => {
                return Err(ValidationError {
                    errors: vec![],
                    parsing_errors: Some(err),
                });
            }
        };

    let errs = get_variable_types(&parsed, types);

    if errs.is_empty() {
        Ok(())
    } else {
        Err(ValidationError {
            errors: errs,
            parsing_errors: None,
        })
    }
}
