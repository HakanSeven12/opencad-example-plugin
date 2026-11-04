//! Reference external add-on, built as a `cdylib` the host loads at runtime.
//!
//! It depends only on `ocs_plugin_api` (with the `host` feature) — never on the
//! `OpenCADStudio` binary — so it demonstrates the stable contract an
//! out-of-tree plugin targets: a `PluginManifest`, a `CadModule` ribbon tab, a
//! quad cursor menu panel (API 15), a `BuiltinPlugin` entry point, and the
//! `export_plugin!` C-ABI export.

use ocs_plugin_api::dialog::{QuadActionDef, QuadItemDef, QuadPanelDef};
use ocs_plugin_api::host::{BuiltinPlugin, Handle, HostApi};
use ocs_plugin_api::manifest::{ApiVersion, PluginManifest};
use ocs_plugin_api::ribbon::{CadModule, IconKind, ModuleEvent, RibbonGroup, RibbonItem, ToolDef};

static MANIFEST: PluginManifest = PluginManifest {
    id: "opencad.example",
    name: "Example Plugin",
    version: env!("CARGO_PKG_VERSION"),
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
    fn ribbon_groups(&self) -> &[RibbonGroup] {
        static GROUPS: std::sync::OnceLock<Vec<RibbonGroup>> = std::sync::OnceLock::new();
        GROUPS.get_or_init(|| {
            vec![RibbonGroup {
                title: "Demo",
                tools: vec![RibbonItem::LargeTool(ToolDef {
                    id: "EX_HELLO",
                    label: "Hello",
                    icon: IconKind::Svg(include_bytes!("../assets/hello.svg")),
                    event: ModuleEvent::Command("EX_HELLO".to_string()),
                })],
            }]
        })
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
    /// API 15: a quad cursor menu panel on lines. "Hello" runs the plugin's
    /// own command; "Count" calls back with the lines the quad points at.
    fn on_load(&mut self, host: &mut dyn HostApi) {
        let registered = host.register_quad_panels(
            MANIFEST.id,
            &[QuadPanelDef {
                panel_id: "example".into(),
                title: "Example".into(),
                applies_to: vec!["Line".into()],
                items: vec![
                    QuadItemDef {
                        control_id: "hello".into(),
                        label: "Hello".into(),
                        icon: IconKind::Svg(include_bytes!("../assets/hello.svg")).into(),
                        action: QuadActionDef::Command("EX_HELLO".into()),
                    },
                    QuadItemDef {
                        control_id: "count".into(),
                        label: "Count".into(),
                        icon: IconKind::Glyph("#").into(),
                        action: QuadActionDef::Callback,
                    },
                ],
            }],
        );
        if registered.iter().any(|ok| !ok) {
            host.push_error("Example Plugin: the quad panel could not be registered.");
        }
    }
    fn on_quad_action(
        &mut self,
        host: &mut dyn HostApi,
        panel_id: &str,
        control_id: &str,
        handles: &[Handle],
        _type_keys: &[&str],
        _pick_point: [f64; 3],
    ) {
        if panel_id == "example" && control_id == "count" {
            host.push_info(&format!("Example Plugin: {} line(s) under the quad.", handles.len()));
        }
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
