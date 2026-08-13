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

--- Returns swap currently in use, in bytes. Never raises. Live -- read fresh
--- from the OS each call (internally throttled).
--- @return number
function sysinfo.get_used_swap() end

--- Returns unused swap space, in bytes. Never raises. Live -- read fresh
--- from the OS each call (internally throttled).
--- @return number
function sysinfo.get_free_swap() end

--- Returns memory currently in use, in bytes. Never raises. Live -- read
--- fresh from the OS each call (internally throttled).
--- @return number
function sysinfo.get_used_memory() end

--- Returns memory not used for anything, in bytes. Never raises. On Linux,
--- prefer get_available_memory() -- this excludes reclaimable disk cache.
--- Live -- read fresh from the OS each call (internally throttled).
--- @return number
function sysinfo.get_free_memory() end

--- Returns memory available for new allocations without swapping, in bytes.
--- Never raises. Live -- read fresh from the OS each call (internally
--- throttled).
--- @return number
function sysinfo.get_available_memory() end

--- Returns seconds since the host booted. Never raises. Live -- read fresh
--- from the OS each call.
--- @return number
function sysinfo.get_uptime() end

--- Returns the host's boot time as a Unix timestamp (seconds since epoch).
--- Never raises. Fixed for the life of the boot, but read fresh each call.
--- @return number
function sysinfo.get_boot_time() end

--- Returns global CPU usage as a percentage (0-100, roughly). Never raises.
--- Computed by diffing against the previous reading, so the first call after
--- the module loads is unreliable (not necessarily 0) -- every call after
--- that is meaningful.
--- @return number
function sysinfo.get_cpu_usage() end

--- Returns the CPU architecture, e.g. "x86_64". Never raises.
--- @return string
function sysinfo.get_cpu_arch() end

--- Despite the name, not Linux-specific: on Linux this is the distribution
--- id (e.g. "ubuntu"); elsewhere it falls back to a normalized platform name
--- ("windows", "macos"). Never raises.
--- @return string
function sysinfo.get_distro_id() end

--- Returns an array of the distribution's closest relatives (e.g. {"debian"}
--- for Ubuntu). Never raises -- an empty table is the normal answer on most
--- non-Linux platforms and plenty of Linux distributions too.
--- @return string[]
function sysinfo.get_distro_id_like() end

--- @class SysinfoLoadAverage
--- @field one number
--- @field five number
--- @field fifteen number

--- Returns the standard 1/5/15-minute load average. Never raises. A genuine
--- load average on every platform (including Windows -- sysinfo samples a
--- real performance counter there, it doesn't approximate from CPU usage).
--- Not the same metric as get_cpu_usage() -- this reflects queue depth, not
--- a CPU-busy percentage, so values above the core count are normal. The
--- first reads after module load (or a fresh boot) are near-zero; that's the
--- moving average ramping up, not a bug.
--- @return SysinfoLoadAverage
function sysinfo.get_load_average() end

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
