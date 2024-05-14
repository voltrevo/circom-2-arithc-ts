import { expect } from 'chai';

import * as c2a from '../src/index';

describe('Circuit', () => {
  before(async () => {
    await c2a.init();
  });

  it('adds two numbers', () => {
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
  
    const outputs = circuit.evalArray(new Uint32Array([3, 5]));
  
    expect(outputs).to.deep.equal(new Uint32Array([8]));
  });

  it('multiplies two numbers', () => {
    const circuit = c2a.Circuit.compile({
      '/src/main.circom': `
        pragma circom 2.0.0;
  
        // Two element sum
        template sum () {
          signal input a;
          signal input b;
          signal output out;
  
          out <== a * b;
        }
  
        component main = sum();
      `
    });
  
    const outputs = circuit.evalArray(new Uint32Array([3, 5]));
  
    expect(outputs).to.deep.equal(new Uint32Array([15]));
  });
});
