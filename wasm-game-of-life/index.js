import { Universe, Cell } from "./pkg/wasm_game_of_life.js";

const initUniverse = (wasm) => {
  const GRID_COLOR = "#CCCCCC";
  const DEAD_COLOR = "#FFFFFF";
  const ALIVE_COLOR = "#000000";
  const GRID_WIDTH = 450;
  const GRID_HEIGHT = 450;
  const CELL_SIZE = 10; // px
  let rendering = false;
  const universe = Universe.new(GRID_WIDTH, GRID_HEIGHT);
  const width = universe.width();
  const height = universe.height();

  const canvas = document.getElementById("wasm-game-of-life-canvas");
  const renderButton = document.getElementById("render-btn");
  const clearButton = document.getElementById("clear-btn");

  canvas.height = (CELL_SIZE + 1) * height + 1;
  canvas.width = (CELL_SIZE + 1) * width + 1;
  const ctx = canvas.getContext('2d');

  const renderLoop = () => {
    draw();

    if (rendering) {
      universe.tick();
      requestAnimationFrame(renderLoop);
    }
  };


  const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= width; i++) {
      ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
      ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
      ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
      ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
  }

  const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(wasm.memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
      for (let col = 0; col < width; col++) {
        const idx = universe.get_index(row, col);

        ctx.fillStyle = cells[idx] === Cell.Dead
          ? DEAD_COLOR
          : ALIVE_COLOR;

        ctx.fillRect(
          col * (CELL_SIZE + 1) + 1,
          row * (CELL_SIZE + 1) + 1,
          CELL_SIZE,
          CELL_SIZE
        );
      }
    }

    ctx.stroke();
  }

  const draw = () => {
    drawGrid();
    drawCells();
  };

  const getMousePos = (canvas, evt) => {
    const rect = canvas.getBoundingClientRect();
    return {
      x: evt.clientX - rect.left,
      y: evt.clientY - rect.top
    };
  };

  const getHoveredSquare = (mousePos) => {
    let col = Math.floor(mousePos.x / CELL_SIZE);
    let row = Math.floor(mousePos.y / CELL_SIZE);

    return { row, col };
  };


  canvas.addEventListener('mousedown', () => {
    canvas.addEventListener('mousemove', onMouseMove);
  });

  canvas.addEventListener('mouseup', () => {
    canvas.removeEventListener('mousemove', onMouseMove);
  });

  const onMouseMove = (evt) => {
    const mousePos = getMousePos(canvas, evt);
    const hoveredSquare = getHoveredSquare(mousePos);
    console.log(hoveredSquare);
    universe.set_alive(hoveredSquare.row, hoveredSquare.col)
    requestAnimationFrame(renderLoop);
  };

  renderButton.addEventListener('click', () => {
    rendering = !rendering;
    renderButton.innerHTML = rendering ? 'stop' : 'start';
    requestAnimationFrame(renderLoop);
  });

  clearButton.addEventListener('click', () => {
    universe.clear();
    requestAnimationFrame(renderLoop);
  });


  const wrapper = document.getElementById('scroll-container');
  const dragItem = wrapper.querySelector('canvas');
  let isDragging = false;
  let lastX, lastY;

  wrapper.addEventListener('mousedown', function(e) {
    isDragging = true;
    lastX = e.clientX;
    lastY = e.clientY;
  });
  wrapper.addEventListener('mousemove', function(e) {
    if (isDragging) {
      var deltaX = e.clientX - lastX;
      var deltaY = e.clientY - lastY;
      var left = parseInt(dragItem.style.left) || 0;
      var top = parseInt(dragItem.style.top) || 0;
      dragItem.style.left = (left + deltaX) + 'px';
      dragItem.style.top = (top + deltaY) + 'px';
      lastX = e.clientX;
      lastY = e.clientY;
    }
  });
  wrapper.addEventListener('mouseup', function(e) {
    isDragging = false;
  });
  wrapper.addEventListener('mouseleave', function(e) {
    isDragging = false;
  });

  wrapper.addEventListener('wheel', function(e) {
    e.preventDefault();
    var delta = e.deltaY;
    var scale = parseFloat(dragItem.style.transform.replace('scale(', '').replace(')', '')) || 1;
    var newScale = scale + (delta * -0.01);
    if (newScale < 0.1) {
      newScale = 0.1;
    }
    dragItem.style.transform = 'scale(' + newScale + ')';
  });


  requestAnimationFrame(renderLoop);
};

export {
  initUniverse
};
