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
