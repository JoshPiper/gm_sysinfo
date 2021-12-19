
console.log("Loading Toolchain Data")

const {PLATFORM: platform, ARCH: arch, REALM: realm} = process.env
const modname = process.env["MODULE"] ?? "sysinfo"

console.log(`Configuring ${modname} for ${platform}${arch} on ${realm}`)

const archs = {
	["32"]: "i686",
	["64"]: "x86_64"
}

const platforms = {
	win: "pc-windows-gnu",
	osx: "apple-darwin",
	linux: "unknown-linux-gnu"
}

const suffixes = {
	linux32: "linux"
}

const features = {
	cl: 'gmcl'
}

const toolchain = `${archs[arch]}-${platforms[platform]}`
console.log(`::set-output name=toolchain::${toolchain}`)

const file = `gm${realm}_${modname}_${suffixes[`${platform}${arch}`] ?? `${platform}${arch}`}.dll`
console.log(`::set-output name=file::${file}`)

let feature = features[realm] ?? ""
if (feature !== ""){
	feature = `--features ${feature}`
}
console.log(`::set-output name=features::${feature}`)
