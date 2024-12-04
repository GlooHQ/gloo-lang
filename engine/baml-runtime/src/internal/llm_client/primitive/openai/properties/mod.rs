use internal_llm_client::{
    openai::ResolvedOpenAI, ClientProvider, ResolvedClientProperty, UnresolvedClientProperty,
};

use crate::RuntimeContext;

pub fn resolve_properties(
    provider: &ClientProvider,
    properties: &UnresolvedClientProperty<()>,
    ctx: &RuntimeContext,
) -> anyhow::Result<ResolvedOpenAI> {
    let properties = properties.resolve(provider, &ctx.eval_ctx(false))?;

    let ResolvedClientProperty::OpenAI(props) = properties else {
        anyhow::bail!(
            "Invalid client property. Should have been a openai property but got: {}",
            properties.name()
        );
    };

    Ok(props)
}
