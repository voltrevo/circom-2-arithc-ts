import { getWasmLib } from "./wasmLib";

export default class Circuit {
  #circuitJson: string;

  private constructor(circuitJson: string) {
    this.#circuitJson = circuitJson;
  }

  _debug() {
    console.log(this.#circuitJson);
  }

  eval(_input: Record<string, unknown>): Record<string, unknown> {
    throw new Error('Not implemented');
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
