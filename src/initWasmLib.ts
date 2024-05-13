import * as wasmLib from '../wasm/pkg/circom_2_arithc_ts_wasm.js';

function base64ToUint8Array(base64: string) {
    var binaryString = atob(base64);
    var len = binaryString.length;
    var bytes = new Uint8Array(len);
    for (var i = 0; i < len; i++) {
        bytes[i] = binaryString.charCodeAt(i);
    }
    return bytes;
}

let promise: Promise<typeof wasmLib> | undefined = undefined;

export default function initWasmLib() {
    promise ??= (async () => {
        const { default: wasmBase64 } = await import(
            '../wasm/pkg/circom_2_arithc_ts_wasm_base64.js'
        );
    
        wasmLib.initSync(base64ToUint8Array(wasmBase64));
    
        return wasmLib;
    })();

    return promise;
}
