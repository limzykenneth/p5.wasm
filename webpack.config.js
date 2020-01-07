const path = require("path");
const version = require("./package.json").version;

let publicPath = null;
if(process.env.CDN_BUILD === "true"){
	publicPath = `https://cdn.jsdelivr.net/npm/p5.wasm@${version}/dist/`;
}

module.exports = {
	entry: "./index.js",
	output: {
		path: path.resolve(__dirname, "dist"),
		filename: "p5.wasm.js",
		chunkFilename: "[name].bundle.js",
		webassemblyModuleFilename: "p5.wasm",
		publicPath: publicPath || process.env.ASSETS_PATH || "/wasm/"
	},
	mode: "production"
};
