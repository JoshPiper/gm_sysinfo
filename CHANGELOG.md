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



