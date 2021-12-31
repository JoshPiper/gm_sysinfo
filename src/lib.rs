#![feature(c_unwind)]

#[cfg(feature = "gmcl")]
use gmod::gmcl::override_stdout;
use gmod::lua::{State, LuaInt};
use gmod::lua_function;
use sysinfo::{System, SystemExt};
use lazy_static::initialize;

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate gmod;

static MOD_NAME: &str = "sysinfo";
macro_rules! err {
    () => {format!("{} had an error.", MOD_NAME)};
    ($arg:literal) => {format!("{} was unable to {}", MOD_NAME, $arg)};
    ($arg:literal, $err:literal) => {format!("{} was unable to {}: {:?}", MOD_NAME, $arg, $err)};
}

lazy_static! {
    static ref SYSTEM: System = System::new_all();
    static ref CORES: usize = SYSTEM.physical_core_count().unwrap_or_default();
    static ref TOTAL_MEMORY: u64 = SYSTEM.total_memory();
    static ref TOTAL_SWAP: u64 = SYSTEM.total_swap();
    static ref SYS_NAME: String = SYSTEM.name().unwrap_or_default();
    static ref OS_LONG_VERSION: String = SYSTEM.long_os_version().unwrap_or_default();
    static ref OS_VERSION: String = SYSTEM.os_version().unwrap_or_default();
    static ref KERNEL_VERSION: String = SYSTEM.kernel_version().unwrap_or_default();
    static ref HOST_NAME: String = SYSTEM.host_name().unwrap_or_default();
}

unsafe fn error(lua: State, err: String){
   lua.error(err.as_str());
}

#[lua_function]
unsafe fn get_core_count(lua: State) -> i32 {
    let cores: usize = *CORES;
    if cores == 0 {
        error(lua, err!("read the core count"))
    }

    lua.push_integer(cores as LuaInt);
    1
}

#[lua_function]
unsafe fn get_memory(lua: State) -> i32 {
    let size: u64 = *TOTAL_MEMORY;
    if size == 0 {
        error(lua, err!("read the system memory"))
    }

    lua.push_integer(size as LuaInt);
    1
}

#[lua_function]
unsafe fn get_swap(lua: State) -> i32 {
    let size: u64 = *TOTAL_SWAP;
    if size == 0 {
        error(lua, err!("read the system swap space"))
    }

    lua.push_integer(size as LuaInt);
    1
}

#[lua_function]
unsafe fn get_system_name(lua: State) -> i32 {
    let sys_name: &str = SYS_NAME.as_str();
    if sys_name.is_empty() {
        error(lua, err!("read the system name"))
    }

    lua.push_string(sys_name);
    1
}

#[lua_function]
unsafe fn get_system_long_version(lua: State) -> i32 {
    let version: &str = OS_LONG_VERSION.as_str();
    if version.is_empty() {
        error(lua, err!("read the system version"))
    }

    lua.push_string(version);
    1
}

#[lua_function]
unsafe fn get_system_version(lua: State) -> i32 {
    let version: &str = OS_VERSION.as_str();
    if version.is_empty() {
        error(lua, err!("read the system version"))
    }

    lua.push_string(version);
    1
}

#[lua_function]
unsafe fn get_kernel_version(lua: State) -> i32 {
    let version: &str = KERNEL_VERSION.as_str();
    if version.is_empty() {
        error(lua, err!("read the kernel version"))
    }

    lua.push_string(version);
    1
}

#[lua_function]
unsafe fn get_host_name(lua: State) -> i32 {
    let host_name: &str = HOST_NAME.as_str();
    if host_name.is_empty() {
        error(lua, err!("read the system version"))
    }

    lua.push_string(host_name);
    1
}

#[gmod13_open]
unsafe fn gmod13_open(lua: State) -> i32 {
    macro_rules! export_lua_function {
        ($name:ident) => {
            // _G.sysinfo.$name
            lua.push_function($name);
            lua.set_field(-2, concat!(stringify!($name), "\0").as_ptr() as *const i8);
        }
    }

    #[cfg(feature = "gmcl")]{
        override_stdout();
    }

    // System should be the only thing we need to init here, since that takes the longest to load.
    initialize(&SYSTEM);

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
    println!("Goodbye from binary module!");
    0
}
