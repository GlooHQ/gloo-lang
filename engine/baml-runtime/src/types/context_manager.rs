use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::{Context, Result};
use baml_types::BamlValue;
use std::fmt;

use crate::{client_registry::ClientRegistry, type_builder::TypeBuilder, RuntimeContext, SpanCtx};

use super::runtime_context::BamlSrcReader;

type BamlContext = (uuid::Uuid, String, HashMap<String, BamlValue>);

#[derive(Clone)]
pub struct RuntimeContextManager {
    baml_src_reader: Arc<BamlSrcReader>,
    context: Arc<Mutex<Vec<BamlContext>>>,
    env_vars: HashMap<String, String>,
    global_tags: Arc<Mutex<HashMap<String, BamlValue>>>,
}

impl fmt::Debug for RuntimeContextManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RuntimeContextManager")
            .field("context", &self.context.lock())
            .field("global_tags", &self.global_tags)
            .finish()
    }
}

impl RuntimeContextManager {
    pub fn deep_clone(&self) -> Self {
        Self {
            baml_src_reader: self.baml_src_reader.clone(),

            context: Arc::new(Mutex::new(self.context.lock().unwrap().clone())),
            env_vars: self.env_vars.clone(),
            global_tags: Arc::new(Mutex::new(self.global_tags.lock().unwrap().clone())),
        }
    }

    pub fn new_from_env_vars(
        env_vars: HashMap<String, String>,
        baml_src_reader: BamlSrcReader,
    ) -> Self {
        Self {
            baml_src_reader: Arc::new(baml_src_reader),
            context: Default::default(),
            env_vars,
            global_tags: Default::default(),
        }
    }

    pub fn upsert_tags(&self, tags: HashMap<String, BamlValue>) {
        let mut ctx = self.context.lock().unwrap();
        if let Some((.., last_tags)) = ctx.last_mut() {
            last_tags.extend(tags);
        } else {
            self.global_tags.lock().unwrap().extend(tags);
        }
    }

    fn clone_last_tags(&self) -> HashMap<String, BamlValue> {
        self.context
            .lock()
            .unwrap()
            .last()
            .map(|(_, _, tags)| tags.clone())
            .unwrap_or_default()
    }

    pub fn enter(&self, name: &str) -> uuid::Uuid {
        let last_tags = self.clone_last_tags();
        let span = uuid::Uuid::new_v4();
        self.context
            .lock()
            .unwrap()
            .push((span, name.to_string(), last_tags));
        log::trace!("Entering with: {:#?}", self.context.lock().unwrap());
        span
    }

    pub fn exit(&self) -> Option<(uuid::Uuid, Vec<SpanCtx>, HashMap<String, BamlValue>)> {
        let mut ctx = self.context.lock().unwrap();
        log::trace!("Exiting: {:#?}", ctx);

        let prev = ctx
            .iter()
            .map(|(span, name, _)| SpanCtx {
                span_id: *span,
                name: name.clone(),
            })
            .collect();

        let (id, _, mut tags) = ctx.pop()?;

        for (k, v) in self.global_tags.lock().unwrap().iter() {
            tags.entry(k.clone()).or_insert_with(|| v.clone());
        }

        Some((id, prev, tags))
    }

    pub fn create_ctx(
        &self,
        type_builder: Option<&TypeBuilder>,
        client_registry: Option<&ClientRegistry>,
    ) -> Result<RuntimeContext> {
        let mut tags = self.global_tags.lock().unwrap().clone();
        let ctx_tags = {
            self.context
                .lock()
                .unwrap()
                .last()
                .map(|(.., x)| x)
                .cloned()
                .unwrap_or_default()
        };
        tags.extend(ctx_tags);
        let tags = {
            let ctx = self.context.lock().unwrap();
            let ctx = ctx.last();
            ctx.map(|(.., tags)| tags).cloned().unwrap_or_default()
        };

        let (cls, enm, als, rec_als) = type_builder
            .map(TypeBuilder::to_overrides)
            .unwrap_or_default();

        let mut ctx = RuntimeContext::new(
            self.baml_src_reader.clone(),
            self.env_vars.clone(),
            tags,
            Default::default(),
            cls,
            enm,
            als,
            rec_als,
        );

        ctx.client_overrides = match client_registry {
            Some(cr) => Some(
                cr.to_clients(&ctx)
                    .with_context(|| "Failed to create clients from client_registry")?,
            ),
            None => None,
        };

        Ok(ctx)
    }

    pub fn create_ctx_with_default(&self) -> RuntimeContext {
        let ctx = self.context.lock().unwrap();

        RuntimeContext::new(
            self.baml_src_reader.clone(),
            self.env_vars.clone(),
            ctx.last().map(|(.., x)| x).cloned().unwrap_or_default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        )
    }

    pub fn context_depth(&self) -> usize {
        let ctx = self.context.lock().unwrap();
        ctx.len()
    }
}
