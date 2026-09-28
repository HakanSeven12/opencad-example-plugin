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

> Approach B: the binary must be built with the same `rustc` and against the
> same `opencadcodec` commit as the host. `Cargo.toml` pins `ocs_plugin_api` to
> an OpenCADStudio commit, which also pins its `opencadcodec`; the release
> workflow writes the compiler and the resolved `opencadcodec` source into the
> shipped `plugin.toml` (`rustc_version`, `acadrust_source`), and the host
> refuses a plugin whose fingerprint differs from its own.
>
> To follow a new Open CAD Studio release: set the `rev` in `Cargo.toml` to that
> release's commit, run `cargo update -p ocs_plugin_api`, bump the version, and
> tag with the same stable toolchain the host release was built with.
>
> Release `v0.4.0` targets plugin API 7. API 2 is no longer loaded by current
> hosts.

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
