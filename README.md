# circom-2-arithc(-ts)

TypeScript library for compiling circom to arithmetic circuits backed by
[namnc/circom-2-arithc](https://github.com/namnc/circom-2-arithc).

## Usage

```sh
npm install circom-2-arithc
```

```ts
import * as c2a from 'circom-2-arithc';

async function main() {
  await c2a.init();

  const circuitSrc = {
    // In a real project you should be able to include these as regular files,
    // but how those files find their way into this format depends on your build
    // tool.

    '/src/main.circom': `
      pragma circom 2.0.0;

      template Adder() {
          signal input a, b;
          signal output c;

          c <== a + b;
      }

      component main = Adder();
    `,
  };

  const circuit = c2a.Circuit.compile(circuitSrc);

  console.log(
    // In future named inputs and outputs should work via
    // `circuit.eval({ a: 3, b: 5 })`
    circuit.evalArray(new Uint32Array([3, 5])),
  ); // [8]

  // For use with mpz-ts, use `circuit.toMpzCircuit()` (not yet implemented).
}

main().catch(console.error);
```

## Development

Build with `npm run build`. This will compile the wasm subproject and also
transpile typescript into javascript. [Rust toolchain](https://rustup.rs/)
required.

Test with `npm test`.

## Example Project

For a more complete MPC example in the form of a repository, see
[mpz-ts-example](https://github.com/voltrevo/mpz-ts-example).
