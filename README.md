# opencad-example-plugin

A reference **external** add-on for [Open CAD Studio](https://github.com/HakanSeven12/OpenCADStudio),
distributed as a prebuilt dynamic library via GitHub Releases.

It depends only on `ocs_plugin_api` (the host's stable contract crate), never on
the OpenCADStudio binary, and exports the two C symbols the host loader expects
(via `ocs_plugin_api::export_plugin!`). It adds an **Example** ribbon tab and an
`EX_HELLO` command.

## Install (from Open CAD Studio)

In OCS: **Plugin Manager → Add repository →** `HakanSeven12/opencad-example-plugin`,
pick a compatible release from the dropdown, and click **Install**. Restart OCS;
the **Example** tab appears and `EX_HELLO` responds.

## Releases

Pushing a `v*` tag runs `.github/workflows/release.yml`, which builds the cdylib
on Linux/Windows/macOS and uploads each binary plus `plugin.toml` as release
assets. The host picks the asset matching the user's OS/arch and reads
`plugin.toml` for the API-version compatibility check.

> Approach B: the binary must be built against the same toolchain and
> `ocs_plugin_api` version as the host. The `ocs_plugin_api_version` symbol
> gates the API version at load time.

## Get listed in Open CAD Studio

To make a plugin discoverable in the OCS **Plugin Manager → Available plugins**
(so users install it without typing a repo), open a pull request adding an entry
to [`plugins/registry.json`](https://github.com/HakanSeven12/OpenCADStudio/blob/main/plugins/registry.json)
in the OpenCADStudio repo:

```json
{
  "repo": "your-account/your-plugin-repo",
  "name": "Human-readable name",
  "description": "One line describing what it does."
}
```

See [`plugins/README.md`](https://github.com/HakanSeven12/OpenCADStudio/blob/main/plugins/README.md)
for the requirements. The registry is fetched from `main` at runtime, so a
merged PR reaches every user without an app update.

This repo is itself the first registry entry — use it as the template for your
own plugin and release workflow.
