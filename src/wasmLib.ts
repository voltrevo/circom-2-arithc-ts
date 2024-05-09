import { initSync } from '../wasm/pkg/circom_2_arithc_ts_wasm.js';
import wasmBase64 from '../wasm/pkg/circom_2_arithc_ts_wasm_base64.js';

function base64ToUint8Array(base64: string) {
    var binaryString = atob(base64);
    var len = binaryString.length;
    var bytes = new Uint8Array(len);
    for (var i = 0; i < len; i++) {
        bytes[i] = binaryString.charCodeAt(i);
    }
    return bytes;
}

export default initSync(base64ToUint8Array(wasmBase64));
