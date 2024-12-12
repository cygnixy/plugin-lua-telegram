use cygnixy_plugin_interface::{export_plugin, PluginLua};
use mlua::prelude::LuaError;
use mlua::{Function, Lua};
use reqwest::blocking::Client;
use std::collections::HashMap;
use std::error::Error;
use tracing::{debug, error, info, trace, warn};

pub struct PluginLuaTelegram;

impl Default for PluginLuaTelegram {
    fn default() -> Self {
        Self
    }
}

impl PluginLuaTelegram {
    pub fn new() -> Self {
        PluginLuaTelegram
    }
}

impl PluginLua for PluginLuaTelegram {
    fn name(&self) -> &str {
        "telegram"
    }

    fn on_load(&mut self) -> Result<(), Box<dyn Error>> {
        trace!("PluginLuaTelegram loaded!");
        Ok(())
    }

    fn on_unload(&mut self) -> Result<(), Box<dyn Error>> {
        trace!("PluginLuaTelegram unloaded!");
        Ok(())
    }

    fn get_lua_functions(&self, lua: &Lua, name: &str) -> HashMap<String, Function> {
        let mut functions = HashMap::new();

        if let Err(e) = self.register_functions(lua, name, &mut functions) {
            error!("Failed to register Lua functions: {}", e);
        }

        functions
    }
}

fn send_telegram_message(token: &str, chat_id: &str, text: &str) -> Result<(), Box<dyn Error>> {
    if token.trim().is_empty() {
        return Ok(());
    }
    if chat_id.trim().is_empty() {
        return Ok(());
    }
    if text.trim().is_empty() {
        return Ok(());
    }
    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);
    let client = Client::new();

    let _ = client
        .post(&url)
        .form(&[("chat_id", chat_id), ("text", text)])
        .send()?;

    Ok(())
}

impl PluginLuaTelegram {
    fn register_functions(
        &self,
        lua: &Lua,
        name: &str,
        functions: &mut HashMap<String, Function>,
    ) -> Result<(), Box<dyn Error>> {
        // Registering the "telegram.send" function
        functions.insert(
            "send".to_string(),
            lua.create_function(|_, (token, chat_id, text): (String, String, String)| {
                let _ = send_telegram_message(&token, &chat_id, &text);
                Ok(())
            })?,
        );

        Ok(())
    }
}

export_plugin!(PluginLuaTelegram);
