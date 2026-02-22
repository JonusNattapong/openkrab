//! wasm_runtime — WebAssembly plugin runtime using Wasmtime.
//!
//! Provides cross-platform plugin execution with WASI support.

use anyhow::{anyhow, bail, Context, Result};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};
use wasmtime::component::ResourceTable;
use wasmtime::{Config, Engine, Instance, Linker, Module, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};

use crate::plugin_sdk::{PluginContext, PluginDeclaration, PluginTool};
use crate::plugins::sandbox::{Sandbox, SandboxConfig, SandboxLevel};
use crate::plugins::{HookPhase, HookSlots, PluginHook};

/// WASM plugin instance state
pub struct WasmPluginState {
    /// WASI context for filesystem/network access
    wasi: WasiCtx,
    /// WASI resource table
    table: ResourceTable,
    /// Plugin metadata
    pub name: String,
    pub version: String,
    /// Registered tools
    pub tools: Vec<PluginTool>,
    /// Hook phases this plugin registered
    pub hook_phases: Vec<String>,
    /// Storage for plugin data
    pub storage: HashMap<String, Vec<u8>>,
}

impl WasiView for WasmPluginState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}

impl WasmPluginState {
    pub fn new(name: String, version: String) -> Self {
        let wasi = WasiCtxBuilder::new().inherit_stdio().inherit_env().build();

        Self {
            wasi,
            table: ResourceTable::new(),
            name,
            version,
            tools: Vec::new(),
            hook_phases: Vec::new(),
            storage: HashMap::new(),
        }
    }

    /// Add a tool to this plugin's registry
    pub fn register_tool(&mut self, tool: PluginTool) {
        self.tools.push(tool);
    }

    /// Register a hook phase
    pub fn register_hook(&mut self, phase: String) {
        if !self.hook_phases.contains(&phase) {
            self.hook_phases.push(phase);
        }
    }
}

/// A loaded WASM plugin
pub struct WasmPlugin {
    pub name: String,
    pub version: String,
    pub engine: Engine,
    pub module: Module,
    pub declaration: Option<PluginDeclaration>,
    /// Pre-instantiated store for reuse
    store: Arc<RwLock<Store<WasmPluginState>>>,
    /// Sandbox configuration
    pub sandbox: Sandbox,
}

impl WasmPlugin {
    /// Load a WASM plugin from a file with sandboxing
    pub async fn load_with_sandbox(path: &Path, sandbox: Sandbox) -> Result<Self> {
        info!("Loading WASM plugin from {} with sandbox", path.display());

        // Configure WASM engine with resource limits
        let mut config = Config::new();
        config.async_support(true);
        config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);

        // Apply memory limit from sandbox
        let mem_limit = sandbox.resources().max_memory;
        config.static_memory_maximum_size(mem_limit as u64);

        let engine = Engine::new(&config)?;
        let module = Module::from_file(&engine, path)
            .with_context(|| format!("Failed to load WASM module from {}", path.display()))?;

        // Try to extract declaration from custom sections
        let declaration = Self::extract_declaration(&module);

        let name = declaration
            .as_ref()
            .map(|d| d.name.clone())
            .unwrap_or_else(|| {
                path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string()
            });

        let version = declaration
            .as_ref()
            .map(|d| d.version.clone())
            .unwrap_or_else(|| "1.0.0".to_string());

        // Create initial store
        let state = WasmPluginState::new(name.clone(), version.clone());
        let store = Store::new(&engine, state);

        let plugin = Self {
            name,
            version,
            engine,
            module,
            declaration,
            store: Arc::new(RwLock::new(store)),
            sandbox,
        };

        // Initialize the plugin with sandbox
        plugin.initialize().await?;

        info!(
            "WASM plugin '{}' v{} loaded successfully with sandbox",
            plugin.name, plugin.version
        );
        Ok(plugin)
    }

    /// Load a WASM plugin from a file (without sandbox - for backward compatibility)
    pub async fn load(path: &Path) -> Result<Self> {
        let sandbox = Sandbox::new("wasm-plugin", SandboxConfig::medium());
        Self::load_with_sandbox(path, sandbox).await
    }

    /// Extract plugin declaration from WASM custom sections
    fn extract_declaration(module: &Module) -> Option<PluginDeclaration> {
        // Look for a custom section named "plugin_declaration"
        for section in module.custom_sections() {
            if section.name() == "plugin_declaration" {
                let data = section.data();
                if let Ok(json_str) = std::str::from_utf8(data) {
                    if let Ok(declaration) = serde_json::from_str::<PluginDeclaration>(json_str) {
                        return Some(declaration);
                    }
                }
            }
        }
        None
    }

    /// Initialize the plugin instance
    async fn initialize(&self) -> Result<()> {
        let mut store = self.store.write().await;

        // Set up WASI linker
        let mut linker = Linker::new(&self.engine);
        wasmtime_wasi::add_to_linker_async(&mut linker)?;

        // Add krabkrab-specific host functions
        Self::add_host_functions(&mut linker)?;

        // Instantiate the module
        let instance = linker
            .instantiate_async(&mut *store, &self.module)
            .await
            .context("Failed to instantiate WASM module")?;

        // Call the initialize function if it exists
        if let Ok(init_fn) = instance.get_typed_func::<(), ()>(&mut *store, "krabkrab_init") {
            init_fn
                .call_async(&mut *store, ())
                .await
                .context("Plugin initialization failed")?;
        }

        // Extract tools and hooks from the instance
        self.extract_capabilities(&instance, &mut store).await?;

        Ok(())
    }

    /// Add krabkrab host functions to the linker
    fn add_host_functions(linker: &mut Linker<WasmPluginState>) -> Result<()> {
        // Log function for plugins
        linker.func_wrap(
            "krabkrab",
            "log",
            |mut caller: wasmtime::Caller<'_, WasmPluginState>, ptr: i32, len: i32| {
                let memory = caller
                    .get_export("memory")
                    .and_then(|e| e.into_memory())
                    .ok_or_else(|| anyhow::anyhow!("Memory not found"))?;

                let mut buf = vec![0u8; len as usize];
                memory.read(&caller, ptr as usize, &mut buf)?;

                if let Ok(message) = std::str::from_utf8(&buf) {
                    log::info!("[WASM plugin] {}", message);
                }

                Ok(())
            },
        )?;

        // Register tool function
        linker.func_wrap(
            "krabkrab",
            "register_tool",
            |mut caller: wasmtime::Caller<'_, WasmPluginState>,
             name_ptr: i32,
             name_len: i32,
             desc_ptr: i32,
             desc_len: i32,
             schema_ptr: i32,
             schema_len: i32| {
                let memory = caller
                    .get_export("memory")
                    .and_then(|e| e.into_memory())
                    .ok_or_else(|| anyhow::anyhow!("Memory not found"))?;

                let name = read_string_from_memory(&memory, &caller, name_ptr, name_len)?;
                let description = read_string_from_memory(&memory, &caller, desc_ptr, desc_len)?;
                let schema_json =
                    read_string_from_memory(&memory, &caller, schema_ptr, schema_len)?;

                let parameters: serde_json::Value = serde_json::from_str(&schema_json)
                    .unwrap_or_else(|_| serde_json::json!({"type": "object", "properties": {}}));

                let tool = PluginTool {
                    name: name.clone(),
                    description,
                    parameters,
                };

                caller.data_mut().register_tool(tool);
                log::debug!("WASM plugin registered tool: {}", name);

                Ok(())
            },
        )?;

        // Register hook function
        linker.func_wrap(
            "krabkrab",
            "register_hook",
            |mut caller: wasmtime::Caller<'_, WasmPluginState>, phase_ptr: i32, phase_len: i32| {
                let memory = caller
                    .get_export("memory")
                    .and_then(|e| e.into_memory())
                    .ok_or_else(|| anyhow::anyhow!("Memory not found"))?;

                let phase = read_string_from_memory(&memory, &caller, phase_ptr, phase_len)?;
                caller.data_mut().register_hook(phase.clone());
                log::debug!("WASM plugin registered hook: {}", phase);

                Ok(())
            },
        )?;

        // Storage get function
        linker.func_wrap(
            "krabkrab",
            "storage_get",
            |mut caller: wasmtime::Caller<'_, WasmPluginState>,
             key_ptr: i32,
             key_len: i32,
             out_ptr: i32,
             out_cap: i32,
             out_len: i32|
             -> i32 {
                let memory = match caller.get_export("memory").and_then(|e| e.into_memory()) {
                    Some(m) => m,
                    None => return -1,
                };

                let key = match read_string_from_memory(&memory, &caller, key_ptr, key_len) {
                    Ok(k) => k,
                    Err(_) => return -1,
                };

                if let Some(data) = caller.data().storage.get(&key) {
                    let len = data.len().min(out_cap as usize);
                    if let Err(_) = memory.write(&mut caller, out_ptr as usize, &data[..len]) {
                        return -1;
                    }
                    if let Err(_) =
                        memory.write(&mut caller, out_len as usize, &(len as i32).to_le_bytes())
                    {
                        return -1;
                    }
                    len as i32
                } else {
                    -1
                }
            },
        )?;

        // Storage set function
        linker.func_wrap(
            "krabkrab",
            "storage_set",
            |mut caller: wasmtime::Caller<'_, WasmPluginState>,
             key_ptr: i32,
             key_len: i32,
             val_ptr: i32,
             val_len: i32| {
                let memory = caller
                    .get_export("memory")
                    .and_then(|e| e.into_memory())
                    .ok_or_else(|| anyhow::anyhow!("Memory not found"))?;

                let key = read_string_from_memory(&memory, &caller, key_ptr, key_len)?;
                let mut value = vec![0u8; val_len as usize];
                memory.read(&caller, val_ptr as usize, &mut value)?;

                caller.data_mut().storage.insert(key, value);

                Ok(())
            },
        )?;

        Ok(())
    }

    /// Extract capabilities from the instantiated module
    async fn extract_capabilities(
        &self,
        instance: &Instance,
        store: &mut Store<WasmPluginState>,
    ) -> Result<()> {
        // Call capability export if available
        if let Ok(cap_fn) =
            instance.get_typed_func::<(), (i32, i32)>(&mut *store, "krabkrab_capabilities")
        {
            let (ptr, len) = cap_fn.call_async(&mut *store, ()).await?;

            if let Some(memory) = instance.get_memory(&mut *store, "memory") {
                let mut buf = vec![0u8; len as usize];
                memory.read(&*store, ptr as usize, &mut buf)?;

                if let Ok(json) = std::str::from_utf8(&buf) {
                    if let Ok(declaration) = serde_json::from_str::<PluginDeclaration>(json) {
                        // Register tools
                        for tool in &declaration.tools {
                            store.data_mut().register_tool(tool.clone());
                        }
                        // Register hooks
                        for phase in &declaration.hook_phases {
                            store.data_mut().register_hook(phase.clone());
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Call a tool in this WASM plugin
    pub async fn call_tool(
        &self,
        tool_name: &str,
        args: serde_json::Value,
        ctx: &PluginContext,
    ) -> Result<serde_json::Value> {
        let mut store = self.store.write().await;

        // Get the tool call function
        let instance = self.get_instance(&mut store).await?;

        let tool_fn = instance
            .get_typed_func::<(i32, i32, i32, i32), (i32, i32)>(
                &mut *store,
                &format!("tool_{}", tool_name),
            )
            .with_context(|| format!("Tool '{}' not found in plugin '{}'", tool_name, self.name))?;

        // Serialize arguments and context
        let args_json = serde_json::to_string(&args)?;
        let ctx_json = serde_json::to_string(&ctx)?;

        // Allocate memory in WASM for input
        let args_ptr = self
            .allocate_string(&mut store, &instance, &args_json)
            .await?;
        let ctx_ptr = self
            .allocate_string(&mut store, &instance, &ctx_json)
            .await?;

        // Call the tool
        let (result_ptr, result_len) = tool_fn
            .call_async(
                &mut *store,
                (
                    args_ptr,
                    args_json.len() as i32,
                    ctx_ptr,
                    ctx_json.len() as i32,
                ),
            )
            .await
            .with_context(|| format!("Tool '{}' execution failed", tool_name))?;

        // Read result
        let result = self
            .read_string(&store, &instance, result_ptr, result_len)
            .await?;

        // Deallocate memory
        self.deallocate(&mut store, &instance, args_ptr).await?;
        self.deallocate(&mut store, &instance, ctx_ptr).await?;
        self.deallocate(&mut store, &instance, result_ptr).await?;

        // Parse result
        let result_json: serde_json::Value =
            serde_json::from_str(&result).context("Failed to parse tool result")?;

        Ok(result_json)
    }

    /// Execute a hook phase
    pub async fn execute_hook(&self, phase: &HookPhase, ctx: &PluginContext) -> Result<()> {
        let phase_str = format!("{:?}", phase).to_kebab_case();

        let mut store = self.store.write().await;
        let instance = self.get_instance(&mut store).await?;

        let hook_fn_name = format!("hook_{}", phase_str);

        if let Ok(hook_fn) = instance.get_typed_func::<(i32, i32), ()>(&mut *store, &hook_fn_name) {
            let ctx_json = serde_json::to_string(&ctx)?;
            let ctx_ptr = self
                .allocate_string(&mut store, &instance, &ctx_json)
                .await?;

            hook_fn
                .call_async(&mut *store, (ctx_ptr, ctx_json.len() as i32))
                .await
                .with_context(|| format!("Hook '{}' execution failed", hook_fn_name))?;

            self.deallocate(&mut store, &instance, ctx_ptr).await?;
        }

        Ok(())
    }

    /// Get or create instance
    async fn get_instance(&self, store: &mut Store<WasmPluginState>) -> Result<Instance> {
        let mut linker = Linker::new(&self.engine);
        wasmtime_wasi::add_to_linker_async(&mut linker)?;
        Self::add_host_functions(&mut linker)?;

        let instance = linker.instantiate_async(&mut *store, &self.module).await?;
        Ok(instance)
    }

    /// Allocate a string in WASM memory
    async fn allocate_string(
        &self,
        store: &mut Store<WasmPluginState>,
        instance: &Instance,
        s: &str,
    ) -> Result<i32> {
        let alloc_fn = instance
            .get_typed_func::<i32, i32>(&mut *store, "krabkrab_alloc")
            .context("Allocation function not found")?;

        let ptr = alloc_fn.call_async(&mut *store, s.len() as i32).await?;

        if let Some(memory) = instance.get_memory(&mut *store, "memory") {
            memory.write(&mut *store, ptr as usize, s.as_bytes())?;
        }

        Ok(ptr)
    }

    /// Deallocate memory in WASM
    async fn deallocate(
        &self,
        store: &mut Store<WasmPluginState>,
        instance: &Instance,
        ptr: i32,
    ) -> Result<()> {
        if let Ok(dealloc_fn) = instance.get_typed_func::<i32, ()>(&mut *store, "krabkrab_free") {
            dealloc_fn.call_async(&mut *store, ptr).await?;
        }
        Ok(())
    }

    /// Read a string from WASM memory
    async fn read_string(
        &self,
        store: &Store<WasmPluginState>,
        instance: &Instance,
        ptr: i32,
        len: i32,
    ) -> Result<String> {
        if let Some(memory) = instance.get_memory(&mut store.clone(), "memory") {
            let mut buf = vec![0u8; len as usize];
            memory.read(&*store, ptr as usize, &mut buf)?;
            String::from_utf8(buf).context("Invalid UTF-8 in WASM memory")
        } else {
            bail!("Memory not found")
        }
    }

    /// Get registered tools
    pub async fn tools(&self) -> Vec<PluginTool> {
        let store = self.store.read().await;
        store.data().tools.clone()
    }

    /// Get registered hook phases
    pub async fn hook_phases(&self) -> Vec<String> {
        let store = self.store.read().await;
        store.data().hook_phases.clone()
    }
}

/// Helper to read a string from WASM memory
fn read_string_from_memory(
    memory: &wasmtime::Memory,
    caller: &wasmtime::Caller<'_, WasmPluginState>,
    ptr: i32,
    len: i32,
) -> Result<String> {
    let mut buf = vec![0u8; len as usize];
    memory.read(&caller, ptr as usize, &mut buf)?;
    String::from_utf8(buf).context("Invalid UTF-8")
}

/// Convert string to kebab-case
trait ToKebabCase {
    fn to_kebab_case(&self) -> String;
}

impl ToKebabCase for str {
    fn to_kebab_case(&self) -> String {
        self.chars()
            .flat_map(|c| {
                if c.is_uppercase() {
                    vec!['-', c.to_lowercase().next().unwrap_or(c)]
                } else {
                    vec![c]
                }
            })
            .collect::<String>()
            .trim_start_matches('-')
            .to_string()
    }
}

/// WASM plugin manager
pub struct WasmPluginManager {
    plugins: HashMap<String, WasmPlugin>,
}

impl WasmPluginManager {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    /// Load a WASM plugin
    pub async fn load(&mut self, path: &Path) -> Result<String> {
        let plugin = WasmPlugin::load(path).await?;
        let name = plugin.name.clone();
        self.plugins.insert(name.clone(), plugin);
        Ok(name)
    }

    /// Unload a plugin
    pub fn unload(&mut self, name: &str) -> bool {
        self.plugins.remove(name).is_some()
    }

    /// Get a plugin
    pub fn get(&self, name: &str) -> Option<&WasmPlugin> {
        self.plugins.get(name)
    }

    /// Call a tool in a plugin
    pub async fn call_tool(
        &self,
        plugin_name: &str,
        tool_name: &str,
        args: serde_json::Value,
        ctx: &PluginContext,
    ) -> Result<serde_json::Value> {
        let plugin = self
            .plugins
            .get(plugin_name)
            .ok_or_else(|| anyhow!("Plugin '{}' not found", plugin_name))?;

        plugin.call_tool(tool_name, args, ctx).await
    }

    /// Execute hooks for all plugins
    pub async fn execute_hooks(&self, phase: &HookPhase, ctx: &PluginContext) {
        for (name, plugin) in &self.plugins {
            let phases = plugin.hook_phases().await;
            let phase_str = format!("{:?}", phase).to_kebab_case();

            if phases.contains(&phase_str) {
                if let Err(e) = plugin.execute_hook(phase, ctx).await {
                    warn!("Hook execution failed for plugin '{}': {}", name, e);
                }
            }
        }
    }

    /// Get all tools from all plugins
    pub async fn all_tools(&self) -> HashMap<String, Vec<PluginTool>> {
        let mut result = HashMap::new();
        for (name, plugin) in &self.plugins {
            let tools = plugin.tools().await;
            if !tools.is_empty() {
                result.insert(name.clone(), tools);
            }
        }
        result
    }

    /// List loaded plugins
    pub fn list(&self) -> Vec<&str> {
        self.plugins.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for WasmPluginManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kebab_case() {
        assert_eq!("BeforeAgentStart".to_kebab_case(), "before-agent-start");
        assert_eq!("AfterToolCall".to_kebab_case(), "after-tool-call");
        assert_eq!("on-session-end".to_kebab_case(), "on-session-end");
    }
}
