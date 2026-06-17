//! Reference external add-on, built as a `cdylib` the host loads at runtime.
//!
//! It depends only on `ocs_plugin_api` (with the `host` feature) — never on the
//! `OpenCADStudio` binary — so it demonstrates the stable contract an
//! out-of-tree plugin targets: a `PluginManifest`, a `CadModule` ribbon tab, a
//! `BuiltinPlugin` entry point, and the `export_plugin!` C-ABI export.

use ocs_plugin_api::host::{BuiltinPlugin, HostApi};
use ocs_plugin_api::manifest::{ApiVersion, PluginManifest};
use ocs_plugin_api::ribbon::{CadModule, IconKind, ModuleEvent, RibbonGroup, RibbonItem, ToolDef};

static MANIFEST: PluginManifest = PluginManifest {
    id: "opencad.example",
    name: "Example Plugin",
    version: "0.1.0",
    description: "Reference dynamically-loaded add-on",
    api_version: ApiVersion::CURRENT,
    ribbon_order: 50,
    xdata_apps: &[],
    command_prefixes: &["EX_"],
};

/// Ribbon tab for the example plugin.
struct ExampleModule;

impl CadModule for ExampleModule {
    fn id(&self) -> &'static str {
        "example"
    }
    fn title(&self) -> &'static str {
        "Example"
    }
    fn ribbon_groups(&self) -> Vec<RibbonGroup> {
        vec![RibbonGroup {
            title: "Demo",
            tools: vec![RibbonItem::LargeTool(ToolDef {
                id: "EX_HELLO",
                label: "Hello",
                icon: IconKind::Glyph("◆"),
                event: ModuleEvent::Command("EX_HELLO".to_string()),
            })],
        }]
    }
}

/// The plugin entry point handed to the host.
struct ExamplePlugin;

impl BuiltinPlugin for ExamplePlugin {
    fn manifest(&self) -> &'static PluginManifest {
        &MANIFEST
    }
    fn ribbon(&self) -> Box<dyn CadModule> {
        Box::new(ExampleModule)
    }
    fn dispatch(&self, host: &mut dyn HostApi, cmd: &str) -> bool {
        match cmd {
            "EX_HELLO" => {
                host.push_info("Hello from the external example plugin (cdylib loaded).");
                true
            }
            _ => false,
        }
    }
}

// Emit the C-ABI symbols the host loader looks for.
ocs_plugin_api::export_plugin!(ExamplePlugin);
