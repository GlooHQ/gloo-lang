use magnus::{class, prelude::*, RModule};

use crate::Result;

#[magnus::wrap(class = "Baml::Ffi::RuntimeContextManager", free_immediately, size)]
pub struct RuntimeContextManager {
    pub inner: baml_runtime::RuntimeContextManager,
}
impl RuntimeContextManager {
    pub fn define_in_ruby(module: &RModule) -> Result<()> {
        let cls = module.define_class("RuntimeContextManager", class::object())?;
        // cls.define_method(
        //     "set_cancelled",
        //     method!(RuntimeContextManager::set_cancelled, 0),
        // )?;
        //cls.define_method("upsert_tags", method!(RuntimeContextManager::upsert_tags, 1))?;
        //cls.define_method("deep_clone", method!(RuntimeContextManager::deep_clone, 0))?;

        Ok(())
    }

    fn set_cancelled(&self) {
        self.inner.create_ctx_with_default().cancel();
    }
}
