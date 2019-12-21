import(/* webpackChunkName: "index" */ "../pkg").then((wasm) => {
	p5.prototype.wasm = wasm;
});
