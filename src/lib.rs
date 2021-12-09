#![feature(c_unwind)]

#[cfg(feature = "gmcl")]
use gmod::gmcl::override_stdout;
use gmod::lua::State;
use gmod::lua_function;
// use sysinfo::{System, Processor, Disk};

#[macro_use] extern crate gmod;

#[lua_function]
unsafe fn test_print(lua: State) -> i32 {
    lua.get_global(lua_string!("print"));
    lua.push_string("Hello from binary function!");
    lua.call(1, 0);
    0
}

#[gmod13_open]
unsafe fn gmod13_open(lua: State) -> i32 {
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

    // println!("Hello from binary module!");
    // lua.get_global(lua_string!("print"));
    // lua.push_string("Hello from binary module!");
    // lua.call(1, 0);


    // lua.get_global(lua_string!("PrintTable"));

    // lua.create_table(0, 3);
    // lua.push_boolean(true);
    // lua.set_field(-2, lua_string!("from_rust"));

    // lua.push_integer(69);
    // lua.set_field(-2, lua_string!("funny_number"));

    // lua.create_table(2, 0);
    // lua.push_integer(10);
    // lua.raw_seti(-2, 1);

    // lua.push_integer(100);
    // lua.raw_seti(-2, 2);

    // lua.set_field(-2, lua_string!("nested_table"));
    // lua.call(1, 0);

    lua.push_function(test_print);
    lua.set_global(lua_string!("sysinfo_print"));

    0
}

#[gmod13_close]
fn gmod13_close(_lua: State) -> i32 {
    println!("Goodbye from binary module!");
    0
}
