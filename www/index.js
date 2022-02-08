import * as wasm from "wasm-demo";

wasm.greet();

global.wasmCall = wasm.call

global.wasm = wasm
