let canvas = document.getElementById('canvas'); //Get the base canvas

// Returns an object containing functions that can be called in from Rust.
function imports() {
  var context = canvas.getContext('2d');

  function clear_screen() {
    console.log("Clearing screen")
    //Clear the screen, draw the blank grid with random cells here.
  }

  function draw_dead_cell(x,y) {
    console.log("Drawing dead cell at", x, y);
    //Draw the cell here
  }

  function draw_living_cell(x,y) {
    console.log("Drawing living cell at", x, y);
  }

  let imports =  { clear_screen, draw_dead_cell, draw_living_cell };
  return imports;
}

fetch('wasm/life.wasm').then(response => 
  response.arrayBuffer()
).then(bytes =>
  WebAssembly.instantiate(bytes, { env: imports() })
).then(results => {
  let module = {}
  let mod = results.instance;
  module.update_state = mod.exports.update_state;
  module.resize = mod.exports.resize;
  module.draw = mod.exports.draw;

  //Resizing
  function resize() {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    module.resize(canvas.width, canvas.height); //Doing this would also bring the grid to its original state.
  }
  window.addEventListener('resize', () => {
    resize();
  });

  //Main "Game" loop
  let start = null;
  let prevTimestamp = null;
  let drawAndUpdateState = (timestamp) => {
    console.log("Current timestamp is", timestamp); //TODO Maybe get rid of this or display it in the canvas.
    //Initialize state
    if (!prevTimestamp) {
      start = timestamp;
      prevTimestamp = timestamp;
      requestAnimationFrame(drawAndUpdateState);
      return;
    }

    //We first draw and then update
    module.draw();
    module.update_state();
    requestAnimationFrame(drawAndUpdateState);
  };

  resize();
  drawAndUpdateState();
});