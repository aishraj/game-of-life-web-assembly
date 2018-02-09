fetch("life.wasm", {
  cache: "no-cache"
}).then(response =>
  response.arrayBuffer()
).then(bytes =>
  WebAssembly.instantiate(bytes, {})
).then(results => {
  let module = {};
  let mod = results.instance;
  module.alloc = mod.exports.alloc;
  module.dealloc = mod.exports.dealloc;
  module.nextGeneration = mod.exports.next_generation;
  module.clear = mod.exports.clear;

  let width = 1512;
  let height = 1512;

  let byteSize = width * height * 4;
  let pointer = module.alloc(byteSize);
  let buffer = new Uint8Array(mod.exports.memory.buffer, pointer, byteSize);

  let button = document.getElementById("run-wasm");
  let canvas = document.getElementById('screen');
  if (canvas.getContext) {
    let ctx = canvas.getContext('2d');

    let pointer = module.alloc(byteSize);

    let usub = new Uint8ClampedArray(mod.exports.memory.buffer, pointer, byteSize);
    let img = new ImageData(usub, width, height);

    let running = false;
    let i = 0;
    function step(timestamp) {
      console.log("Stepping" ,i);
      i += 1;
      if (!running) return;
      let usub = new Uint8ClampedArray(mod.exports.memory.buffer, pointer, byteSize);
      let img = new ImageData(usub, width, height);
      ctx.putImageData(img, 0, 0);
      let nextGenerationPointer = module.nextGeneration(pointer, width, height);
      pointer = nextGenerationPointer;
      window.requestAnimationFrame(step);
    }

    function clearCanvasAndRestart() {
      running = false;
      window.requestAnimationFrame(function () {
        ctx.clearRect(0, 0, width, height);
        module.clear(pointer, width, height);
        running = true;
        window.requestAnimationFrame(step);
      });
    }

    clearCanvasAndRestart();
  }
});