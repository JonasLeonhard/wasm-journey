console.log("loaded");

import init from "./pkg/wasm_game_of_life.js";

init().then(async (wasm) => {
  const { initUniverse } = await import('./index.js');
  initUniverse(wasm);
});
