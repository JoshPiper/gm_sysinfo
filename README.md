# gm_sysinfo

Fetching System Information in Rust to Lua.

## Installation

Download a copy of the module from the releases (or compile from source)
Move the downloaded file to <Garry's Mod Installation>/lua/bin/gm<state>_sysinfo_<platform>.dll

State can be either cl or sv, for the client and server, respectively.
Platform can be one of win32, win64, linux or linux64 for 32 bit (main branch) Windows, 64 bit (x64 branch) Windows, and 32/64 bit Linux builds respectively. 


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
  
### `sysinfo.get_memory(): int`
Returns amount of system memory in KB.
  
### `sysinfo.get_swap(): int`
Returns amount of swap space in KB.
  
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
  
## Credits

Massive thanks to [Billy](https://github.com/WilliamVenner) for both [gmod-rs](https://github.com/WilliamVenner/gmod-rs) and his infinate patience in dealing with both me and my issues. Without him, this project wouldn't have happened.
