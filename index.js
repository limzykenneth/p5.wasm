/*global p5*/
import(/* webpackChunkName: "index" */ "p5-wasm").then((wasm) => {
	p5.prototype.wasm = wasm;
});
