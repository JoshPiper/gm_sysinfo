const childProcess = require("child_process")
const exec = childProcess.spawnSync

exports.preCommit = ({tag, version}) => {
	const {GITHUB_REPOSITORY} = process.env
	let name = GITHUB_REPOSITORY.split("/")[1]

	let res = exec("cargo", ["update", "-p", name])
	console.log(res)
}
