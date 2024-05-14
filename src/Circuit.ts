import { getWasmLib } from "./wasmLib";

export default class Circuit {
  #circuitJson: string;

  private constructor(circuitJson: string) {
    this.#circuitJson = circuitJson;
  }

  toJson() {
    return JSON.parse(this.#circuitJson);
  }

  eval(_input: Record<string, unknown>): Record<string, unknown> {
    throw new Error(
      'Not implemented: named inputs and outputs (use evalArray)'
    );
  }

  evalArray(inputs: Uint32Array): Uint32Array {
    return getWasmLib().eval_(this.#circuitJson, inputs);
  }

  toMpzCircuit(): { mpzCircuitData(): Uint8Array } {
    return {
      mpzCircuitData: () => {
        throw new Error('Not implemented');
      },
    };
  }

  static compile(files: Record<string, string>): Circuit {
    return new Circuit(getWasmLib().compile(files));
  }
}
