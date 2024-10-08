use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use plugin::{Plugin, PluginRef};

pub mod cacher;
pub mod plugin;

// --- Internal Types ---

pub(crate) type ArRw<T> = Arc<RwLock<T>>;

// --- Re-exports ---

pub use bugi_core::*;
#[allow(unused_imports)]
pub use bugi_share::*;

#[cfg(feature = "plug-host")]
pub use bugi_host::*;

// --- Universe ---

/// Stores plugins
#[derive(Clone)]
pub struct Universe(ArRw<UniverseInner>);

/// Inner data of Universe
struct UniverseInner {
    plugins: HashMap<PluginId, Arc<Plugin>>,
    next_id: PluginId,
}

impl Universe {
    /// Create a new Universe
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(UniverseInner {
            plugins: HashMap::new(),
            next_id: 0,
        })))
    }

    /// Add a plugin to the Universe
    pub fn add_plugin_raw(&self, plugin: Plugin) -> Result<PluginRef, BugiError> {
        let mut inner = self.0.write().unwrap();

        // Check ID
        // TODO: This is O(n) and can be optimized to use a HashSet
        // Modify this if performance becomes an issue
        for (_, p) in inner.plugins.iter() {
            if p.get_str_id() == plugin.get_str_id() {
                return Err(BugiError::PluginIdExists(plugin.get_str_id().to_string()));
            }
        }

        let id = inner.next_id;
        inner.next_id += 1;

        let plugin = Arc::new(plugin);

        inner.plugins.insert(id, Arc::clone(&plugin));
        Ok(PluginRef::new(Arc::downgrade(&plugin)))
    }

    /// add plugin with PluginSystem
    pub fn add_plugin(
        &self,
        str_id: &str,
        detail: impl bugi_core::PluginSystem + 'static,
    ) -> Result<PluginRef, BugiError> {
        self.add_plugin_raw(Plugin::new(str_id, detail))
    }
}

impl Default for Universe {
    fn default() -> Self {
        Self::new()
    }
}
