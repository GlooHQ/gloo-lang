use crate::forms::{Argument, Expression, Identifier, Type};

pub struct LLMFunction<T> {
    pub name: Identifier<T>,
    pub args: Vec<Argument<T>>,
    pub return_type: Type<T>,
    pub body: LLMFunctionBody<T>,
}

pub struct LLMFunctionBody<T> {
    pub client: Expression<T>,
    pub prompt: Expression<T>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::FunctionParser;
    use internal_baml_diagnostics::{Diagnostics, SourceFile};

    #[test]
    fn test_parse_function() {

        let source_file = SourceFile::new_static("tmp.baml".into(), "");
        let p = FunctionParser::new();

        let mut diagnostics = Diagnostics::new("tmp.baml".into());
        p.parse(&source_file, &mut diagnostics, r##"
          function Foo(a: int, b: string | int) -> bool {
            client SomClient
            prompt #"
              An example prompt template string.
            "#
          }
        "##).unwrap();
    }

}
