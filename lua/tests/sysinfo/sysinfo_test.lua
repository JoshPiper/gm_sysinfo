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
            name = "Reports swap as positive bytes, or raises when absent",
            func = function()
                -- Containers commonly have no swap; the API contract is to
                -- raise a Lua error rather than return 0.
                local ok, swap = pcall( sysinfo.get_swap )
                if ok then
                    expect( swap ).to.beGreaterThan( 0 )
                end
            end
        },
        {
            name = "Reports system identity strings",
            func = function()
                expect( sysinfo.get_system_name() ).to.beA( "string" )
                expect( sysinfo.get_kernel_version() ).to.beA( "string" )
                expect( sysinfo.get_host_name() ).to.beA( "string" )
            end
        },
    }
}
