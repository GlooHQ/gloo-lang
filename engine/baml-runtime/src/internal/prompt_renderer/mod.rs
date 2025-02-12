mod render_output_format;
use internal_llm_client::ClientSpec;
use jsonish::{BamlValueWithFlags, ResponseBamlValue};
use render_output_format::render_output_format;

use anyhow::Result;
use baml_types::{BamlValue, FieldType};
use internal_baml_core::{
    error_unsupported,
    ir::{repr::IntermediateRepr, FunctionWalker, IRHelper},
};
use internal_baml_jinja::{
    types::OutputFormatContent, RenderContext, RenderContext_Client, RenderedPrompt,
    TemplateStringMacro,
};

use crate::RuntimeContext;

use super::llm_client::parsed_value_to_response;

pub struct PromptRenderer {
    function_name: String,
    client_spec: ClientSpec,
    output_defs: OutputFormatContent,
    output_type: FieldType,
}

impl PromptRenderer {
    pub fn from_function(
        function: &FunctionWalker,
        ir: &IntermediateRepr,
        ctx: &RuntimeContext,
    ) -> Result<PromptRenderer> {
        let func_v2 = function.elem();
        let Some(config) = func_v2.configs.first() else {
            error_unsupported!("function", function.name(), "no valid prompt found")
        };

        Ok(PromptRenderer {
            function_name: function.name().into(),
            client_spec: match &ctx.client_overrides {
                Some((Some(client), _)) => ClientSpec::Named(client.clone()),
                _ => config.client.clone(),
            },
            output_defs: render_output_format(ir, ctx, &func_v2.output)?,
            output_type: func_v2.output.clone(),
        })
    }

    pub fn client_spec(&self) -> &ClientSpec {
        &self.client_spec
    }

    pub fn parse(
        &self,
        ir: &IntermediateRepr,
        raw_string: &str,
        allow_partials: bool
    ) -> Result<ResponseBamlValue> {
        let parsed = jsonish::from_str(
            &self.output_defs,
            &self.output_type,
            raw_string,
            allow_partials,
        )?;
        let res = parsed_value_to_response(ir, parsed, &self.output_type, allow_partials);
        res
    }

    pub fn render_prompt(
        &self,
        ir: &IntermediateRepr,
        ctx: &RuntimeContext,
        params: &BamlValue,
        client_ctx: &RenderContext_Client,
    ) -> Result<RenderedPrompt> {
        let func = ir.find_function(&self.function_name)?;

        let func_v2 = func.elem();

        let Some(config) = func_v2.configs.first() else {
            error_unsupported!("function", self.function_name, "no valid prompt found")
        };

        internal_baml_jinja::render_prompt(
            &config.prompt_template,
            params,
            RenderContext {
                client: client_ctx.clone(),
                tags: ctx.tags.clone(),
                output_format: self.output_defs.clone(),
            },
            &ir.walk_template_strings()
                .map(|t| TemplateStringMacro {
                    name: t.name().into(),
                    args: t
                        .inputs()
                        .iter()
                        .map(|i| (i.name.clone(), i.r#type.elem.to_string()))
                        .collect(),
                    template: t.template().into(),
                })
                .collect::<Vec<_>>(),
            ir,
            ctx.env_vars(),
        )
    }
}
