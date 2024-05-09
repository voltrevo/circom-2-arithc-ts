export default class Circuit {
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

  static compile(_files: Record<string, string>): Circuit {
    throw new Error('Not implemented');
  }
}
