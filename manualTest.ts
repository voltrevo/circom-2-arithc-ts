import { initWasmLib } from "./src/wasmLib";

async function main() {
    const src = `
        pragma circom 2.0.0;

        // Two element sum
        template sum () {
            signal input a;
            signal input b;
            signal output out;
            
            out <== a + b;
        }
        
        component main = sum();
    `;

    const wasmLib = await initWasmLib();

    const circuit = wasmLib.compile(src);

    console.log(circuit);
}

main().catch(console.error);
