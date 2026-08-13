use std::sync::LazyLock;

use gmod::lua::{LuaString, State};
use gmod::{gmod13_close, gmod13_open, lua_function, lua_string};
use sysinfo::{MemoryRefreshKind, RefreshKind, System};

static MOD_NAME: &str = "sysinfo";
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
    if INFO.total_swap == 0 {
        error(lua, err!("read the system swap space"));
    }

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
    lua.create_table(0, 8);
    export_lua_function!(get_core_count);
    export_lua_function!(get_memory);
    export_lua_function!(get_swap);
    export_lua_function!(get_system_name);
    export_lua_function!(get_system_long_version);
    export_lua_function!(get_system_version);
    export_lua_function!(get_kernel_version);
    export_lua_function!(get_host_name);
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
