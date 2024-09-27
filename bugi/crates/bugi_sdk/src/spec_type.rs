use anyhow::Result;
use serde::{Deserialize, Serialize};
use wasmtime::{Memory, Store};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PluginDesc {
    pub(crate) string_id: String,
}

impl PluginDesc {
    pub(crate) fn ser(mem: (&Memory, &Store<()>), ptr: i32, len: i32) -> Result<Self> {
        let data = &mem.0.data(mem.1)[(ptr as usize)..((ptr + len) as usize)];
        let data = rmp_serde::from_slice::<PluginDesc>(data)?;
        Ok(data)
    }
}
