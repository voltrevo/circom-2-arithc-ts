import * as c2a from "./src/index";

async function main() {
  await c2a.init();

  const circuit = c2a.Circuit.compile({
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
  });

  const [output] = circuit.evalArray(new Uint32Array([3, 5]));

  console.log(output);
}

main().catch(console.error);
