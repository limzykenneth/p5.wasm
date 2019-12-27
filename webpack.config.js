const path = require("path");

module.exports = {
	entry: "./index.js",
	output: {
		path: path.resolve(__dirname, "dist"),
		filename: "p5.wasm.js",
		chunkFilename: "[name].bundle.js",
		webassemblyModuleFilename: "p5.wasm",
		publicPath: process.env.ASSETS_PATH || "/wasm/"
	},
	mode: "production"
};
