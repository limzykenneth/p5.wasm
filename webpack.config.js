/* eslint-env node*/
const path = require("path");

module.exports = {
	entry: "./index.js",
	output: {
		path: path.resolve(__dirname, "dist"),
		filename: "bootstrap.js",
		chunkFilename: "[name].bundle.js",
		publicPath: process.env.ASSETS_PATH || "/wasm/"
	},
	mode: "production"
};
