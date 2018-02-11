let canvas = document.getElementById('canvas'); //Get the base canvas

// Returns an object containing functions that can be called in from Rust.
function imports() {
  var context = canvas.getContext('2d');
  context.strokeStyle = '#e1e1e1';
  context.fillStyle = 'cadetblue';

  function clear_screen() {
    //Clear the screen, draw the blank grid with random cells here.
    context.clearRect(0, 0, 1512, 512);
  }

  function draw_dead_cell(x,y) {
    context.beginPath();
    context.rect(x*8, y*8, 8, 8);
    context.stroke();
  }

  function draw_living_cell(x,y) {
    context.beginPath();
    context.rect(x*8, y*8, 8, 8);
    context.fill();
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
  module.calculate_next_gen = mod.exports.calculate_next_gen;
  module.resize = mod.exports.resize;
  module.draw = mod.exports.draw;

  //Resizing
  function resize() {
    //TODO: Remove hardcoded values
    canvas.width = 1512;
    canvas.height = 1512;
    module.resize(64, 64); //Doing this would also bring the grid to its original state.
  }
  window.addEventListener('resize', () => {
    resize();
  });

  //Main "Game" loop
  let start = null;
  let prevTimestamp = null;
  let drawAndUpdateState = (timestamp) => {
    //console.log("Current timestamp is", timestamp); //TODO Maybe get rid of this or display it in the canvas.
    //Initialize state
    if (!prevTimestamp) {
      start = timestamp;
      prevTimestamp = timestamp;
      window.requestAnimationFrame(drawAndUpdateState);
      //setTimeout(function() {drawAndUpdateState(10);});
      return;
    }

    //We first draw and then update
    module.draw();
    module.calculate_next_gen();
    module.update_state();
    window.requestAnimationFrame(drawAndUpdateState);
    //setTimeout(function() {drawAndUpdateState(10);}, 100);
  };

  resize();
  drawAndUpdateState();
});