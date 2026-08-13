# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [3.0.0](https://github.com/JoshPiper/gm_sysinfo/compare/v2.0.0...v3.0.0) - 2026-08-13

### Added

- add sysinfo.get_load_average()
- add sysinfo.get_cpu_usage(), get_cpu_arch(), get_distro_id(), get_distro_id_like()
- add sysinfo.get_uptime() and sysinfo.get_boot_time()
- add sysinfo.get_used_memory(), get_free_memory(), get_available_memory()
- add sysinfo.get_used_swap() and sysinfo.get_free_swap()
- add throttled refresh infrastructure for dynamic metrics
- embed build provenance and ship supply-chain artifacts

### Fixed

- use toNot.beLessThan(0) instead of beGreaterThan(-1) in the load average test
- use toNot.beLessThan(0) instead of beGreaterThan(-1) in the CPU usage test
- use toNot.beLessThan(0) instead of beGreaterThan(-1) in the uptime test
- use toNot.beLessThan(0) instead of beGreaterThan(-1) in the used/free swap test
- use toNot.beLessThan(0) instead of beGreaterThan(-1) in the swap test
- [**breaking**] get_swap() no longer raises when no swap is configured

### Other

- drop inter-bracket spacing in the load average test
- drop inter-bracket spacing in the CPU/distro tests
- prune the get_distro_id_like comment per CONTRIBUTING.md
- drop inter-bracket spacing in the uptime test
- drop inter-bracket spacing in the memory test
- drop inter-bracket spacing in the used/free swap test
- cut the get_swap comment to one line
- prune the get_swap comment per CONTRIBUTING.md
- tighten the Cache doc comment further
- prune the Cache doc comment per CONTRIBUTING.md
- space the LuaLS description comments too
- use "--- @tag" spacing in the LuaLS annotations
- Refine language and formatting in README.md
- rewrite README, ship a LuaLS type stub, round out crate metadata
# [2.0.0](https://github.com/JoshPiper/gm_sysinfo/compare/v1.0.6...v2.0.0) (2026-08-13)


* feat!: modernize toolchain and dependencies ([bf710f9](https://github.com/JoshPiper/gm_sysinfo/commit/bf710f9df4ee7cfa87c56cf2bb6ed08bf2add962))


### BREAKING CHANGES

* get_memory() and get_swap() now return bytes
(previously KiB) as Lua numbers rather than integers.

Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>



## [1.0.6](https://github.com/JoshPiper/gm_sysinfo/compare/v1.0.5...v1.0.6) (2022-07-01)


### Bug Fixes

* **security:** Updated Outdated Dependency. ([fca7e56](https://github.com/JoshPiper/gm_sysinfo/commit/fca7e565173e96c61e2f9244b2e5b5fc61c077be))



## [1.0.5](https://github.com/JoshPiper/gm_sysinfo/compare/v1.0.4...v1.0.5) (2021-12-31)


### Bug Fixes

* **build:** Checkout on the tagged version, not the pre-tag version. ([0435de0](https://github.com/JoshPiper/gm_sysinfo/commit/0435de0cc71b9f59a2dca09653ab3c51feaf4100))



## [1.0.4](https://github.com/JoshPiper/gm_sysinfo/compare/v1.0.3...v1.0.4) (2021-12-31)


### Bug Fixes

* **build:** Automatically bump the cargo lock file. ([d7de695](https://github.com/JoshPiper/gm_sysinfo/commit/d7de695f6fc68db643d398db9dc10d59f0e76068))



## [1.0.3](https://github.com/JoshPiper/gm_sysinfo/compare/v1.0.2...v1.0.3) (2021-12-31)


### Bug Fixes

* **deps:** Update minimum version of gmod-rs, adds fix for client console crashing. ([083bfdb](https://github.com/JoshPiper/gm_sysinfo/commit/083bfdbafc1e36bc023325d4d510c8264da9a172))



