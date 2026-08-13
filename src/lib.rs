use std::sync::{LazyLock, Mutex};
use std::time::Instant;

use gmod::lua::{LuaString, State};
use gmod::{gmod13_close, gmod13_open, lua_function, lua_string};
use sysinfo::{
    CpuRefreshKind, MemoryRefreshKind, RefreshKind, System, MINIMUM_CPU_UPDATE_INTERVAL,
};

mod build_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

static MOD_NAME: &str = "sysinfo";
#[cfg(feature = "gmcl")]
static REALM: &str = "cl";
#[cfg(not(feature = "gmcl"))]
static REALM: &str = "sv";

macro_rules! err {
    ($arg:literal) => {
        format!("{} was unable to {}", MOD_NAME, $arg)
    };
}

/// System facts snapshotted once, on module load. Sizes are in bytes.
struct Snapshot {
    cores: usize,
    total_memory: u64,
    total_swap: u64,
    name: String,
    long_os_version: String,
    os_version: String,
    kernel_version: String,
    host_name: String,
}

static INFO: LazyLock<Snapshot> = LazyLock::new(|| {
    let sys = System::new_with_specifics(
        RefreshKind::nothing().with_memory(MemoryRefreshKind::everything()),
    );
    Snapshot {
        cores: System::physical_core_count().unwrap_or_default(),
        total_memory: sys.total_memory(),
        total_swap: sys.total_swap(),
        name: System::name().unwrap_or_default(),
        long_os_version: System::long_os_version().unwrap_or_default(),
        os_version: System::os_version().unwrap_or_default(),
        kernel_version: System::kernel_version().unwrap_or_default(),
        host_name: System::host_name().unwrap_or_default(),
    }
});

unsafe fn error<S: AsRef<str>>(lua: State, err: S) -> ! {
    lua.error(err)
}

/// Throttled per kind, to allow others to refresh. The minimum sysinfo
/// upstream recommended polling rate is `MINIMUM_CPU_UPDATE_INTERVAL`,
/// reused here for memory too.
struct Cache {
    system: System,
    last_memory_refresh: Option<Instant>,
    last_cpu_refresh: Option<Instant>,
}

static CACHE: LazyLock<Mutex<Cache>> = LazyLock::new(|| {
    Mutex::new(Cache {
        system: System::new(),
        last_memory_refresh: None,
        last_cpu_refresh: None,
    })
});

#[allow(dead_code)] // consumed starting with the swap-used-free layer
fn with_memory<T>(f: impl FnOnce(&System) -> T) -> T {
    let mut cache = CACHE.lock().unwrap_or_else(|e| e.into_inner());
    if cache
        .last_memory_refresh
        .is_none_or(|t| t.elapsed() >= MINIMUM_CPU_UPDATE_INTERVAL)
    {
        cache
            .system
            .refresh_memory_specifics(MemoryRefreshKind::everything());
        cache.last_memory_refresh = Some(Instant::now());
    }
    f(&cache.system)
}

#[allow(dead_code)] // consumed starting with the cpu-and-distro layer
fn with_cpu<T>(f: impl FnOnce(&System) -> T) -> T {
    let mut cache = CACHE.lock().unwrap_or_else(|e| e.into_inner());
    if cache
        .last_cpu_refresh
        .is_none_or(|t| t.elapsed() >= MINIMUM_CPU_UPDATE_INTERVAL)
    {
        cache
            .system
            .refresh_cpu_specifics(CpuRefreshKind::nothing().with_cpu_usage());
        cache.last_cpu_refresh = Some(Instant::now());
    }
    f(&cache.system)
}

#[lua_function]
unsafe fn get_core_count(lua: State) -> i32 {
    if INFO.cores == 0 {
        error(lua, err!("read the core count"));
    }

    lua.push_number(INFO.cores as f64);
    1
}

#[lua_function]
unsafe fn get_memory(lua: State) -> i32 {
    if INFO.total_memory == 0 {
        error(lua, err!("read the system memory"));
    }

    lua.push_number(INFO.total_memory as f64);
    1
}

#[lua_function]
unsafe fn get_swap(lua: State) -> i32 {
    // 0 is legitimate here -- unlike memory, plenty of real hosts run with
    // no swap at all, and sysinfo can't tell "no swap" from "failed to
    // read" either way. Erroring on it would misreport a common, valid setup.
    lua.push_number(INFO.total_swap as f64);
    1
}

#[lua_function]
unsafe fn get_system_name(lua: State) -> i32 {
    if INFO.name.is_empty() {
        error(lua, err!("read the system name"));
    }

    lua.push_string(&INFO.name);
    1
}

#[lua_function]
unsafe fn get_system_long_version(lua: State) -> i32 {
    if INFO.long_os_version.is_empty() {
        error(lua, err!("read the system version"));
    }

    lua.push_string(&INFO.long_os_version);
    1
}

#[lua_function]
unsafe fn get_system_version(lua: State) -> i32 {
    if INFO.os_version.is_empty() {
        error(lua, err!("read the system version"));
    }

    lua.push_string(&INFO.os_version);
    1
}

#[lua_function]
unsafe fn get_kernel_version(lua: State) -> i32 {
    if INFO.kernel_version.is_empty() {
        error(lua, err!("read the kernel version"));
    }

    lua.push_string(&INFO.kernel_version);
    1
}

#[lua_function]
unsafe fn get_host_name(lua: State) -> i32 {
    if INFO.host_name.is_empty() {
        error(lua, err!("read the host name"));
    }

    lua.push_string(&INFO.host_name);
    1
}

#[lua_function]
unsafe fn get_version(lua: State) -> i32 {
    lua.push_string(build_info::PKG_VERSION);
    1
}

#[lua_function]
unsafe fn get_build_info(lua: State) -> i32 {
    macro_rules! set_field {
        ($key:literal, $push:expr) => {
            $push;
            lua.set_field(-2, lua_string!($key));
        };
    }
    macro_rules! set_opt_str_field {
        ($key:literal, $value:expr) => {
            match $value {
                Some(v) => lua.push_string(v),
                None => lua.push_nil(),
            }
            lua.set_field(-2, lua_string!($key));
        };
    }

    lua.create_table(0, 10);
    set_field!("version", lua.push_string(build_info::PKG_VERSION));
    set_opt_str_field!("commit", build_info::GIT_COMMIT_HASH);
    set_opt_str_field!("commit_short", build_info::GIT_COMMIT_HASH_SHORT);
    match build_info::GIT_DIRTY {
        Some(dirty) => lua.push_boolean(dirty),
        None => lua.push_nil(),
    }
    lua.set_field(-2, lua_string!("dirty"));
    set_field!("built_at", lua.push_string(build_info::BUILT_TIME_UTC));
    set_field!("target", lua.push_string(build_info::TARGET));
    set_field!("rustc_version", lua.push_string(build_info::RUSTC_VERSION));
    set_field!("realm", lua.push_string(REALM));
    // True when built by a recognised CI platform (currently: GitHub Actions),
    // as opposed to a local `cargo build`. See README for what this promises.
    set_field!(
        "official",
        lua.push_boolean(build_info::CI_PLATFORM.is_some())
    );
    set_field!("repository", lua.push_string(env!("BUILD_REPOSITORY")));
    set_field!("run_url", lua.push_string(env!("BUILD_RUN_URL")));

    1
}

#[gmod13_open]
unsafe fn gmod13_open(lua: State) -> i32 {
    macro_rules! export_lua_function {
        ($name:ident) => {
            // _G.sysinfo.$name
            lua.push_function($name);
            lua.set_field(-2, concat!(stringify!($name), "\0").as_ptr() as LuaString);
        };
    }

    #[cfg(feature = "gmcl")]
    gmod::gmcl::override_stdout();

    // Snapshot the system facts up front, rather than lazily on first call.
    LazyLock::force(&INFO);

    // Create _G.sysinfo
    lua.create_table(0, 10);
    export_lua_function!(get_core_count);
    export_lua_function!(get_memory);
    export_lua_function!(get_swap);
    export_lua_function!(get_system_name);
    export_lua_function!(get_system_long_version);
    export_lua_function!(get_system_version);
    export_lua_function!(get_kernel_version);
    export_lua_function!(get_host_name);
    export_lua_function!(get_version);
    export_lua_function!(get_build_info);
    lua.set_global(lua_string!("sysinfo"));

    0
}

#[gmod13_close]
fn gmod13_close(_lua: State) -> i32 {
    // override_stdout must be undone on unload, or the game crashes.
    #[cfg(feature = "gmcl")]
    gmod::gmcl::restore_stdout();

    0
}
