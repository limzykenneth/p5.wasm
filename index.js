window.p5WasmReady = import(/* webpackChunkName: "index" */ "p5-wasm").then((wasm) => {
	p5.prototype.wasm = wasm.P5Wasm.new();
	return Promise.resolve();
}).catch((e) => {
	console.log(e);
});