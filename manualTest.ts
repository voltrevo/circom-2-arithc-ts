import { initWasmLib } from "./src/wasmLib";

async function main() {
  const files = {
    '/src/main.circom': `
      pragma circom 2.0.0;

      // Two element sum
      template sum () {
        signal input a;
        signal input b;
        signal output out;

        out <== a + b;
      }

      component main = sum();
    `
  };

  const wasmLib = await initWasmLib();
  const circuit = wasmLib.compile(files);

  console.log(circuit);
}

main().catch(console.error);
