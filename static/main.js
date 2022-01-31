import { ImageModule } from "./image_module.js";
import { addConsoleToImports } from "./console.js";

class ConsoleImpl {
  constructor(name) {
    this.name = name;
  }

  consoleLog(msg) {
    console.log(`${this.name}:`, msg);
  }

  consoleError(msg) {
    console.error(`${this.name}:`, msg);
  }
}

// Runs the 'algo' algorithm on the image.
function processImage(algo, module) {
  let img = document.querySelector("img");
  let canvas = document.createElement("canvas");
  let dx = img.naturalWidth;
  let dy = img.naturalHeight;
  canvas.width = dx;
  canvas.height = dy;
  let ctxt = canvas.getContext("2d");

  ctxt.drawImage(img, 0, 0, dx, dy);
  let imageData = ctxt.getImageData(0, 0, dx, dy);

  let start = Date.now();

  let newData = module.processImage(algo, imageData.data, dx, dy);

  alert(`Image processing duration: ${Date.now() - start}ms`);

  let res = new Uint8ClampedArray(newData.buffer, 0, newData.buffer.byteLength);
  ctxt.putImageData(new ImageData(res, dx, dy), 0, 0);

  canvas.toBlob((blob) => {
    img.src = URL.createObjectURL(blob);
  });
}

document.addEventListener("DOMContentLoaded", async () => {
  console.log(`ImageModule is `, ImageModule);
  let module = new ImageModule();

  let imports = {};
  addConsoleToImports(imports, new ConsoleImpl("ImageModule"), (what) => {
    if (what == "memory") {
      return module._exports.memory;
    } else {
      console.error("Unsupport get_export() parameter: ", what);
    }
  });

  await module.instantiate(
    fetch("../target/wasm32-unknown-unknown/release/safeplugins.wasm"),
    imports
  );
  console.log(`Module instanciated`);
  let algorithms = module.algorithms(navigator.language);
  console.log(`Algorithms: `, algorithms.map((algo) => algo.name).join(","));

  let list = document.querySelector("ul");
  let html = "";
  algorithms.forEach((algo) => {
    html += `<li><button data-algo="${algo.name}">${algo.description}</button></li>`;
  });
  list.innerHTML = html;

  list.addEventListener("click", (event) => {
    if (event.target.localName !== "button") {
      return;
    }
    processImage(event.target.dataset.algo, module);
  });

  window.reset.onclick = () => {
    document.querySelector("img").src = "./demo.jpg";
  };
});
