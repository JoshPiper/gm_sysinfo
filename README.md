# gm_sysinfo

Fetching System Information in Rust to Lua.

## Installation

Download a copy of the module from the releases (or compile from source)
Move the downloaded file to `<Garry's Mod Installation>/lua/bin/gm<state>_sysinfo_<platform>.dll`

State can be either cl or sv, for the client and server, respectively.
Platform can be one of win32, win64, linux, linux64 or osx64 for 32 bit (main branch) Windows, 64 bit (x64 branch) Windows, 32/64 bit Linux, and 64 bit (x64 branch) macOS builds respectively.

On macOS, downloaded modules carry the quarantine attribute and may be blocked from loading; clear it with `xattr -d com.apple.quarantine <file>`.


## Usage

```lua
require("sysinfo")
-- Loads _G.sysinfo

local hostname = sysinfo.get_host_name() -- "game_server.example.com"
local cores = sysinfo.get_core_count() -- 8
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
Returns total swap space, in bytes.
  
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
Returns build provenance for the running binary:

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

`commit`, `commit_short`, and `dirty` are `nil` if the binary wasn't built from a git checkout. `repository` and `run_url` are empty strings outside of GitHub Actions. If `official` is `false`, or `run_url` doesn't resolve to a real workflow run, treat the binary as unverified — it wasn't built by this project's release pipeline.

## Verifying a release

Every release binary is built by this repository's GitHub Actions workflow and cryptographically attested. To verify a downloaded file actually came from that pipeline (requires the [GitHub CLI](https://cli.github.com/)):

```bash
gh attestation verify gmsv_sysinfo_linux64.dll -R JoshPiper/gm_sysinfo
```

Each release also ships:
- **`SHA256SUMS`** — checksums for every asset in the release.
- **`gm_sysinfo-<version>.cdx.json`** — a [CycloneDX](https://cyclonedx.org/) software bill of materials for the dependency tree at that version.

Additionally, every binary is built with [`cargo-auditable`](https://github.com/rust-secure-code/cargo-auditable): the exact dependency versions that went into it are embedded in the file itself, so it can be scanned for known vulnerabilities on its own, without needing the SBOM asset or the source repository:

```bash
cargo install cargo-audit --features=binary-scanning
cargo audit bin gmsv_sysinfo_linux64.dll
```

## Credits

Massive thanks to [Billy](https://github.com/WilliamVenner) for both [gmod-rs](https://github.com/WilliamVenner/gmod-rs) and his infinate patience in dealing with both me and my issues. Without him, this project wouldn't have happened.
