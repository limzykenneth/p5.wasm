// Dynamic module loading
if(process.env.ASSETS_PATH){
	__webpack_public_path__ = process.env.ASSETS_PATH;
}else{
	var scriptSrc = document.currentScript.src;
	var filename = scriptSrc.split("/").pop();
	scriptSrc = scriptSrc.replace(filename, "");
	scriptSrc = scriptSrc.replace(window.location.origin, "");
	__webpack_public_path__ = scriptSrc;
}

window.p5WasmReady = import(/* webpackChunkName: "index" */ "p5-wasm").then((wasm) => {
	p5.prototype.wasm = wasm.P5Wasm.new();
	p5.prototype.wasm.Vector = wasm.Vector;
	return Promise.resolve();
}).catch((e) => {
	console.log(e);
});