import { Universe } from "./pkg/wasm_game_of_life.js";

const initUniverse = () => {
  const universe = Universe.new(
    100,
    50,
    document.querySelector("#wasm-game-of-life-canvas")
  );
  const rendering = true;

  const renderLoop = () => {
    universe.draw();

    if (rendering) {
      universe.tick();
      requestAnimationFrame(renderLoop);
    }
  };

  requestAnimationFrame(renderLoop);
};

export { initUniverse };
