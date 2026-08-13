--- @meta

-- Type definitions for gm_sysinfo, for use with the Lua Language Server
-- (https://github.com/LuaLS/lua-language-server). This file declares the
-- shape of the API only; it has no runtime behaviour and must never be
-- require()'d in-game. Point your editor at it instead -- see the
-- "Editor support" section of the README.

--- @class sysinfo
sysinfo = {}

--- Returns the number of physical cores (not threads) on the system.
--- Raises a Lua error if the core count could not be read.
--- @return integer
function sysinfo.get_core_count() end

--- Returns total system memory, in bytes.
--- Raises a Lua error if the memory size could not be read.
--- @return number
function sysinfo.get_memory() end

--- Returns total swap space, in bytes. Returns 0 (never raises) if the host
--- has no swap configured.
--- @return number
function sysinfo.get_swap() end

--- Returns the system name.
--- Raises a Lua error if the system name could not be read.
--- @return string
function sysinfo.get_system_name() end

--- Returns the system DNS name.
--- Raises a Lua error if the host name could not be read.
--- @return string
function sysinfo.get_host_name() end

--- Returns the system version's long name.
--- Raises a Lua error if the version could not be read.
--- @return string
function sysinfo.get_system_long_version() end

--- Returns the system version name.
--- Raises a Lua error if the version could not be read.
--- @return string
function sysinfo.get_system_version() end

--- Returns the kernel version name.
--- Raises a Lua error if the kernel version could not be read.
--- @return string
function sysinfo.get_kernel_version() end

--- Returns the module's own version, e.g. "2.0.0".
--- @return string
function sysinfo.get_version() end

--- @class SysinfoBuildInfo
--- @field version string
--- @field commit string? # nil if not built from a git checkout
--- @field commit_short string? # nil if not built from a git checkout
--- @field dirty boolean? # nil if unknown
--- @field built_at string
--- @field target string
--- @field realm "sv"|"cl"
--- @field rustc_version string
--- @field official boolean # true only for binaries built by this project's CI
--- @field repository string # empty outside of GitHub Actions
--- @field run_url string # empty outside of GitHub Actions

--- Returns build provenance for the running binary. See the README's
--- "Verifying a release" section before trusting `official`, `commit`,
--- or `run_url` for anything security-sensitive.
--- @return SysinfoBuildInfo
function sysinfo.get_build_info() end
