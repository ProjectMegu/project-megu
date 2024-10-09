use std::sync::Weak;

use bugic_core::{BugiError, PluginSystem};
use bugic_share::{FromByte, ParamListTo, SerializeTag};

/// plugin (original)
pub struct Plugin {
    str_id: String,
    detail: Box<dyn PluginSystem>,
}

impl Plugin {
    /// Create a new Host Plugin
    pub fn make_plugin(str_id: &str, detail: impl PluginSystem + 'static) -> Self {
        Self { str_id: str_id.to_string(), detail: Box::new(detail) }
    }

    /// Get the string ID of the plugin
    pub fn get_str_id(&self) -> &String {
        &self.str_id
    }
}

/// Reference to a plugin
pub struct PluginRef {
    /// Weak reference to the plugin of this reference
    pref: Weak<Plugin>,
}

impl PluginRef {
    /// make a new PluginRef
    pub(crate) fn new(pref: Weak<Plugin>) -> Self {
        Self { pref }
    }

    /// Call the plugin
    pub fn call<
        SInput: SerializeTag,
        SOutput: SerializeTag,
        Param: ParamListTo<SInput>,
        Output: FromByte<SOutput>,
    >(
        &self,
        symbol: &str,
        param: Param,
    ) -> Result<Output, BugiError> {
        let plug = self.pref.upgrade().ok_or(BugiError::PluginDropped)?;

        plug.detail
            .check_symbol_abi(symbol, SInput::get_abi_id(), SOutput::get_abi_id())
            .map_err(|(inp, out)| BugiError::PluginAbiError(inp, out))?;

        let param = param.to_byte().map_err(BugiError::CannotSerialize)?;
        let result =
            plug.detail
                .raw_call(symbol, &param, SInput::get_abi_id(), SOutput::get_abi_id())?;
        Ok(Output::from_byte(&result)?)
    }
}
