const path = require("path");
const DefinePlugin = require("webpack").DefinePlugin;
const version = require("./package.json").version;

if(process.env.CDN_BUILD === "true"){
	process.env.ASSETS_PATH = `https://cdn.jsdelivr.net/npm/p5.wasm@${version}/dist/`;
}

module.exports = {
	entry: "./index.js",
	output: {
		path: path.resolve(__dirname, "dist"),
		filename: "p5.wasm.js",
		chunkFilename: "[name].bundle.js",
		webassemblyModuleFilename: "p5.wasm"
	},
	plugins: [
		new DefinePlugin({
			"process.env.ASSETS_PATH": JSON.stringify(process.env.ASSETS_PATH)
		})
	],
	mode: "production"
};
