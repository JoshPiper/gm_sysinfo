return {
    groupName = "sysinfo",

    cases = {
        {
            name = "Loads via require and creates the sysinfo table",
            func = function()
                local ok, err = pcall( require, "sysinfo" )
                if not ok then error( "require('sysinfo') failed: " .. tostring( err ) ) end

                expect( sysinfo ).to.exist()
            end
        },
        {
            name = "Reports a positive physical core count",
            func = function()
                expect( sysinfo.get_core_count() ).to.beGreaterThan( 0 )
            end
        },
        {
            name = "Reports total memory in bytes",
            func = function()
                -- Any real machine or CI container has more than 64 MiB of RAM.
                -- Values below that suggest the old KiB units leaked back in.
                expect( sysinfo.get_memory() ).to.beGreaterThan( 64 * 1024 * 1024 )
            end
        },
        {
            name = "Reports swap as non-negative bytes, never raising",
            func = function()
                -- 0 is a legitimate answer here (no swap configured) rather
                -- than a failure -- unlike the other getters, this must
                -- never raise.
                local swap = sysinfo.get_swap()
                expect(swap).to.beA("number")
                expect(swap).toNot.beLessThan(0)
            end
        },
        {
            name = "Reports used/free swap as non-negative bytes that add up to about the total, never raising",
            func = function()
                local total = sysinfo.get_swap()
                local used = sysinfo.get_used_swap()
                local free = sysinfo.get_free_swap()

                expect(used).to.beA("number")
                expect(free).to.beA("number")
                expect(used).toNot.beLessThan(0)
                expect(free).toNot.beLessThan(0)

                -- Three separate reads, not one atomic snapshot -- allow a
                -- little drift rather than requiring an exact match.
                local drift = math.abs((used + free) - total)
                expect(drift).to.beLessThan(math.max(total * 0.05, 1))
            end
        },
        {
            name = "Reports used/free/available memory as positive bytes, never raising, consistent with total",
            func = function()
                local total = sysinfo.get_memory()
                local used = sysinfo.get_used_memory()
                local free = sysinfo.get_free_memory()
                local available = sysinfo.get_available_memory()

                for _, value in ipairs({ used, free, available }) do
                    expect(value).to.beA("number")
                    expect(value).to.beGreaterThan(0)
                    expect(value).to.beLessThan(total)
                end

                -- available accounts for reclaimable cache that free doesn't,
                -- so it should never be smaller.
                expect(available).to.beGreaterThan(free - 1)
            end
        },
        {
            name = "Reports uptime and boot time consistently, never raising",
            func = function()
                local uptime = sysinfo.get_uptime()
                local boot_time = sysinfo.get_boot_time()

                expect(uptime).to.beA("number")
                expect(boot_time).to.beA("number")
                expect(uptime).toNot.beLessThan(0)

                -- CI runners don't predate the epoch.
                expect(boot_time).to.beGreaterThan(0)
            end
        },
        {
            name = "Reports CPU usage as a number in a sane range, never raising",
            func = function()
                local usage = sysinfo.get_cpu_usage()
                expect(usage).to.beA("number")
                expect(usage).toNot.beLessThan(0)
                expect(usage).to.beLessThan(100 * sysinfo.get_core_count() + 1)
            end
        },
        {
            name = "Reports CPU architecture as a non-empty string, never raising",
            func = function()
                local arch = sysinfo.get_cpu_arch()
                expect(arch).to.beA("string")
                expect(#arch).to.beGreaterThan(0)
            end
        },
        {
            name = "Identity getters return a non-empty string, or raise when unavailable",
            func = function()
                -- Minimal containers may lack e.g. /etc/os-release, in which
                -- case the API contract is to raise, not return "".
                local getters = {
                    "get_system_name",
                    "get_system_version",
                    "get_system_long_version",
                    "get_kernel_version",
                    "get_host_name",
                    "get_cpu_name",
                    "get_cpu_brand",
                }

                for _, name in ipairs( getters ) do
                    local ok, value = pcall( sysinfo[name] )
                    if ok then
                        expect( value ).to.beA( "string" )
                        expect( #value ).to.beGreaterThan( 0 )
                    end
                end
            end
        },
        {
            name = "Reports distro id as a non-empty string, never raising",
            func = function()
                -- CI runs on Linux, so this should be a real distro id --
                -- but the contract holds on any platform.
                local id = sysinfo.get_distro_id()
                expect(id).to.beA("string")
                expect(#id).to.beGreaterThan(0)
            end
        },
        {
            name = "Reports load average as a table of three non-negative numbers, never raising",
            func = function()
                local avg = sysinfo.get_load_average()
                expect(avg).to.exist()

                for _, key in ipairs({ "one", "five", "fifteen" }) do
                    expect(avg[key]).to.beA("number")
                    expect(avg[key]).toNot.beLessThan(0)
                end
            end
        },
        {
            name = "Reports distro id-like as a table of strings, never raising",
            func = function()
                local relatives = sysinfo.get_distro_id_like()
                expect(relatives).to.beA("table")

                for _, relative in ipairs(relatives) do
                    expect(relative).to.beA("string")
                    expect(#relative).to.beGreaterThan(0)
                end
            end
        },
        {
            name = "Reports its own version as a non-empty string",
            func = function()
                local version = sysinfo.get_version()
                expect( version ).to.beA( "string" )
                expect( #version ).to.beGreaterThan( 0 )
            end
        },
        {
            name = "Reports build info matching an official CI build of this target",
            func = function()
                local info = sysinfo.get_build_info()
                expect( info ).to.exist()

                -- These tests only run against binaries built by ci.yml,
                -- so provenance should always resolve.
                expect( info.official ).to.equal( true )
                expect( info.commit ).to.beA( "string" )
                expect( #info.commit ).to.equal( 40 )
                expect( info.dirty ).to.equal( false )
                expect( info.version ).to.equal( sysinfo.get_version() )
                expect( info.repository ).to.beA( "string" )
                expect( #info.repository ).to.beGreaterThan( 0 )
                expect( info.run_url ).to.beA( "string" )
                expect( #info.run_url ).to.beGreaterThan( 0 )

                -- CI always builds server modules on Linux.
                expect( info.realm ).to.equal( "sv" )
                expect( info.target:find( "linux", 1, true ) ).to.exist()
            end
        },
    }
}
