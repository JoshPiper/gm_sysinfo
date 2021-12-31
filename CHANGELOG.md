## [1.0.2](https://github.com/JoshPiper/gm_sysinfo/compare/v1.0.1...v1.0.2) (2021-12-31)


### Bug Fixes

* Allow any stringable to be passed to error. ([4f7921f](https://github.com/JoshPiper/gm_sysinfo/commit/4f7921fecf7fee7b617352cbd8256acc196539da))



## [1.0.1](https://github.com/JoshPiper/gm_sysinfo/compare/v1.0.0...v1.0.1) (2021-12-31)


### Bug Fixes

* **build:** Checkout all commits, to make sure the changelogs properly build. ([7bd762a](https://github.com/JoshPiper/gm_sysinfo/commit/7bd762a82949d3531dd9ca6d0770f6b75180de94))
* call lua.error directly. ([4c37422](https://github.com/JoshPiper/gm_sysinfo/commit/4c37422692f07484e5a0dbd2e0c94a349ebd0556))
* Rewrote internal clone/borrow rules. ([406b1c6](https://github.com/JoshPiper/gm_sysinfo/commit/406b1c6edb1567088d69c0ac4c29d35211aa090b))


### Performance Improvements

* Only deref our lazy static once. ([5977349](https://github.com/JoshPiper/gm_sysinfo/commit/5977349205b69243914ffa230d212eed9173fba3))



# [1.0.0](https://github.com/JoshPiper/gm_sysinfo/compare/43374a3cae09c3f09dc69c079b11296252c355bd...v1.0.0) (2021-12-19)


### Bug Fixes

* allow proper exporting. ([cba0f20](https://github.com/JoshPiper/gm_sysinfo/commit/cba0f20bde37a7b0c4fe14614f71f64e73eca6fc))
* **build:** Don't make a draft. ([e73771c](https://github.com/JoshPiper/gm_sysinfo/commit/e73771cd2740775afcd1c5d724720bdc29b94bce))
* corrected memory precache for sysinfo. ([a7c3126](https://github.com/JoshPiper/gm_sysinfo/commit/a7c3126e4aa084c14fc96411b6492411ad5a1936))
* fix to use the generic name. ([12b92e8](https://github.com/JoshPiper/gm_sysinfo/commit/12b92e83828443cb705d7f5a77a0b7a09e1ec373))


### Documentation

* Add better install instructions. ([11ccdf8](https://github.com/JoshPiper/gm_sysinfo/commit/11ccdf8e114954c84a2125b02b6f1f04fa5d37a8))


### Features

* add a macro for exporting lua functions ([4e77402](https://github.com/JoshPiper/gm_sysinfo/commit/4e77402870ba7560ee6bcb6205102405b495c769))
* add return testing function. ([c0f8034](https://github.com/JoshPiper/gm_sysinfo/commit/c0f8034420f7847fb09013e742a00927347c1e7d))
* Add sysinfo lib ([43374a3](https://github.com/JoshPiper/gm_sysinfo/commit/43374a3cae09c3f09dc69c079b11296252c355bd))
* allow fetching cpu core count. ([0cc2647](https://github.com/JoshPiper/gm_sysinfo/commit/0cc264750c7b81a08dcee90ea0ebe995091e28da))
* Allow passing errors to lua, and getting the hostname. ([921a2cf](https://github.com/JoshPiper/gm_sysinfo/commit/921a2cff4a1bb6f6772705c31bc6c10c5b4cbab9))
* **fix:** final fixes. ([48aef03](https://github.com/JoshPiper/gm_sysinfo/commit/48aef034345e674e0bd473aba95690396659940e))
* **fix:** pass tag to uploader. ([376e4df](https://github.com/JoshPiper/gm_sysinfo/commit/376e4df8c1c06f75d3dbd3933891d64f3c5761d9))
* **fix:** rename from libgm_sysinfo, not gm_sysinfo. ([c42cb9a](https://github.com/JoshPiper/gm_sysinfo/commit/c42cb9a625032deba7e2663318f0c688059373f7))
* **fix:** there's an intermediate folder. ([c783eb1](https://github.com/JoshPiper/gm_sysinfo/commit/c783eb1ac5b9656839f17e8c7ef8172c03da5d25))
* handle core count, total memory, swap space, system name, etc etc. ([b71c1ae](https://github.com/JoshPiper/gm_sysinfo/commit/b71c1aedcf8bd494813644321e556e1c0a4f575a))
* Map get_hostname to _G.sysinfo. ([3e209b1](https://github.com/JoshPiper/gm_sysinfo/commit/3e209b18ab726d2c7fa1669c4aafe57e0199b665))
* rewrite the exporter. ([cc98e86](https://github.com/JoshPiper/gm_sysinfo/commit/cc98e863cdd03a9a53facaf7ba26d0eaff0f8c2e))
* trigger a feature. ([940f21d](https://github.com/JoshPiper/gm_sysinfo/commit/940f21d5bad878c6b0861031614d5a326441d047))


### BREAKING CHANGES

* Initial 1.0.0 Release.



