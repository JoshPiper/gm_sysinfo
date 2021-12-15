#![feature(c_unwind)]

use std::borrow::Borrow;

#[cfg(feature = "gmcl")]
use gmod::gmcl::override_stdout;
use gethostname::gethostname;
use gmod::lua::{State, LuaInt};
use gmod::lua_function;
use sysinfo::{System, Processor, Disk, SystemExt};

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate gmod;

lazy_static! {
    static ref SYSTEM: System = System::new();
    static ref CORES: usize = match SYSTEM.physical_core_count() {
        Some(cores) => cores,
        None => 0
    };
}

unsafe fn error(lua: State, err: String){
    lua.get_global(lua_string!("error"));
    lua.push_string(err.borrow());
    lua.call(1, 0);
}

#[lua_function]
unsafe fn get_hostname(lua: State) -> i32 {
    let hostname = gethostname();
    let hostresult = hostname.into_string();

    let hoststring = match hostresult {
        Err(e) => {
            error(lua, format!("An error occurred in sysinfo whilst fetching the system hostname: {:?}", e));
            "unknown".to_string()
        },
        Ok(host) => host,
    };

    lua.push_string(hoststring.borrow());
    1
}

#[lua_function]
unsafe fn get_core_count(lua: State) -> i32 {
    let cores: usize = CORES.clone();
    match cores {
        0 => {
            error(lua, format!("Sysinfo was unable to read the core count."));
            lua.push_integer(0);
        },
        _ => lua.push_integer(cores as LuaInt)
    }

    1
}

#[lua_function]
unsafe fn test_print(lua: State) -> i32 {
    lua.get_global(lua_string!("print"));
    lua.push_string("Hello from binary function!");
    lua.call(1, 0);
    0
}

#[lua_function]
unsafe fn test_return(lua: State) -> i32 {
    lua.push_string("no u");
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

    println!("This was before the doobery do.");

    #[cfg(feature = "gmcl")]{
        override_stdout();
        println!("Hello client console, from Rust!");
    }

    println!("This has been overwritten.");

    #[cfg(feature = "gmcl")]{
        println!("Compiled for Client");
    }

    println!("This is after the compile block.");

    // Create _G.sysinfo
    lua.create_table(0, 2);
    export_lua_function!(get_hostname);
    export_lua_function!(get_core_count);
    lua.set_global(lua_string!("sysinfo"));

    0
}

#[gmod13_close]
fn gmod13_close(_lua: State) -> i32 {
    println!("Goodbye from binary module!");
    0
}
