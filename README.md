# gm_sysinfo

[![CI](https://github.com/JoshPiper/gm_sysinfo/actions/workflows/ci.yml/badge.svg)](https://github.com/JoshPiper/gm_sysinfo/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/JoshPiper/gm_sysinfo)](https://github.com/JoshPiper/gm_sysinfo/releases/latest)
[![License: MIT](https://img.shields.io/github/license/JoshPiper/gm_sysinfo)](LICENSE)

Fetching System Information in Rust to Lua.

## Installation

Download a copy of the module from the [releases](https://github.com/JoshPiper/gm_sysinfo/releases/latest) page (or [compile from source](#building-from-source)), and move it to:

```
<Garry's Mod Installation>/garrysmod/lua/bin/<filename>
```

`<filename>` follows the pattern `gm<realm>_sysinfo_<platform>.dll`.

| Realm | Platform | Filename |
|---|---|---|
| Server | Windows, 32-bit (`main` branch) | `gmsv_sysinfo_win32.dll` |
| Server | Windows, 64-bit (`x86-64` branch) | `gmsv_sysinfo_win64.dll` |
| Server | Linux, 32-bit (`main` branch) | `gmsv_sysinfo_linux.dll` |
| Server | Linux, 64-bit (`x86-64` branch) | `gmsv_sysinfo_linux64.dll` |
| Server | macOS, 64-bit (`x86-64` branch) | `gmsv_sysinfo_osx64.dll` |
| Client | Windows, 32-bit | `gmcl_sysinfo_win32.dll` |
| Client | Windows, 64-bit | `gmcl_sysinfo_win64.dll` |
| Client | Linux, 32-bit | `gmcl_sysinfo_linux.dll` |
| Client | Linux, 64-bit | `gmcl_sysinfo_linux64.dll` |
| Client | macOS, 64-bit | `gmcl_sysinfo_osx64.dll` |

Server realm exposes the module to server-side Lua; client realm exposes it to the client console/menu. Install whichever (or both) your use case needs.

On macOS, downloaded files carry the quarantine attribute and may be blocked from loading; clear it with `xattr -d com.apple.quarantine <file>`.

## Usage

```lua
require("sysinfo")
-- Loads _G.sysinfo

local hostname = sysinfo.get_host_name() -- "game_server.example.com"
local cores = sysinfo.get_core_count() -- 8
```

An [LuaLS](https://github.com/LuaLS/lua-language-server) type definition file is available - see [Editor support](#editor-support) for autocomplete and inline docs while writing Lua against this module.

## Semantics

- Every value except `get_version()` and `get_build_info()` is **snapshotted once**, when the module loads, not re-read on each call — cheap to call repeatedly, but won't reflect a mid-session change (e.g. hot-added swap).
- Most getters **raise a Lua error** (rather than returning `nil`) if their value couldn't be read at all. A handful return `0` instead where zero is itself a legitimate answer rather than a failure signal — `get_swap()` on a swapless host being the common case — see each function's entry in the API reference below for which rule applies. If you're calling something platform-specific that might not apply to the host you're on, wrap it in `pcall`:

  ```lua
  local ok, value = pcall(sysinfo.get_kernel_version)
  if ok then
      print("Kernel: " .. value)
  end
  ```

## API Reference

### `sysinfo.get_core_count(): int`
Returns the number of physical cores (not threads) on a system.
  
### `sysinfo.get_memory(): number`
Returns total system memory, in bytes.

```lua
local mib = math.floor(sysinfo.get_memory() / 1024 / 1024)
```

### `sysinfo.get_swap(): number`
Returns total swap space, in bytes. Returns `0` (never raises) if the host has no swap configured — that's a legitimate, common state, not a read failure.
  
### `sysinfo.get_system_name(): string`
Returns the system name.
  
### `sysinfo.get_host_name(): string`
Returns the system DNS name.
  
### `sysinfo.get_system_long_version(): string`
Returns the system version long name.
  
### `sysinfo.get_system_version(): string`
Returns the system version name.
  
### `sysinfo.get_kernel_version(): string`
Returns the kernel version name.

### `sysinfo.get_version(): string`
Returns the module's own version, e.g. `"2.0.0"`.

### `sysinfo.get_build_info(): table`
Returns build information for the running binary:

```lua
{
    version       = "2.0.0",
    commit        = "c73b33dc3ad16535a344cd427909c77b79b60bef", -- or nil
    commit_short  = "c73b33d",                                   -- or nil
    dirty         = false,                                       -- or nil if unknown
    built_at      = "Thu, 13 Aug 2026 04:03:15 +0000",
    target        = "x86_64-unknown-linux-gnu",
    realm         = "sv",                                        -- or "cl"
    rustc_version = "rustc 1.99.0-nightly (ad3d0bc14 2026-07-31)",
    official      = true,  -- built by CI, not a local `cargo build`
    repository    = "JoshPiper/gm_sysinfo",
    run_url       = "https://github.com/JoshPiper/gm_sysinfo/actions/runs/123456",
}
```

`commit`, `commit_short`, and `dirty` are `nil` if the binary wasn't built from a git checkout. `repository` and `run_url` are empty strings outside of GitHub Actions. If `official` is `false`, or `run_url` doesn't resolve to a real workflow run, treat the binary as unverified; it wasn't built by this project's release pipeline.

## Editor support

A [LuaLS](https://github.com/LuaLS/lua-language-server) type definition file, [`sysinfo.lua`](sysinfo.lua), ships in this repository and as a release asset. It's declarations only (`---@meta`) — never `require()` it in-game. Point your editor at it instead, e.g. in `.luarc.json`:

```json
{
    "workspace.library": ["path/to/sysinfo.lua"]
}
```

## Building from source

Requires the Rust nightly toolchain pinned in [`rust-toolchain.toml`](rust-toolchain.toml), which `rustup` will automatically install on first build.

```bash
git clone https://github.com/JoshPiper/gm_sysinfo
cd gm_sysinfo
cargo build --release                    # server realm -> target/release/gm_sysinfo.{dll,so,dylib}
cargo build --release --features gmcl    # client realm
```

Cross-compiling to another target needs that target installed (`rustup target add <triple>`) and, for 32-bit Linux specifically, a multilib GCC (`gcc-multilib` on Debian/Ubuntu).
See [`.github/workflows/build.yml`](.github/workflows/build.yml) for the exact target/feature matrix CI builds.
There are some reported pitfalls in cross compiling GMod binaries, so take care during the process.

## Verifying a release

Every release binary is built by this repository's GitHub Actions workflow and cryptographically attested. To verify a downloaded file actually came from that pipeline (requires the [GitHub CLI](https://cli.github.com/)):

```bash
gh attestation verify gmsv_sysinfo_linux64.dll -R JoshPiper/gm_sysinfo
```

Each release also ships:
- **`SHA256SUMS`** - checksums for every asset in the release.
- **`gm_sysinfo-<version>.cdx.json`** - a [CycloneDX](https://cyclonedx.org/) software bill of materials for the dependency tree at that version.

Additionally, every binary is built with [`cargo-auditable`](https://github.com/rust-secure-code/cargo-auditable): the exact dependency versions that went into it are embedded in the file itself, so it can be scanned for known vulnerabilities on its own, without needing the SBOM asset or the source repository:

```bash
cargo install cargo-audit --features=binary-scanning
cargo audit bin gmsv_sysinfo_linux64.dll
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## Security

See [SECURITY.md](SECURITY.md) for how to report a vulnerability.

## Credits

Massive thanks to [Billy](https://github.com/WilliamVenner) for both [gmod-rs](https://github.com/WilliamVenner/gmod-rs) and his infinate patience in dealing with both me and my issues. Without him, this project wouldn't have happened.
