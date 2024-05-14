import { initWasmLib } from './wasmLib';

export { default as Circuit } from './Circuit';

export async function init() {
  await initWasmLib();
}
