import init, { setup_message, greet } from '../../physics/pkg/physics.js';


const greetTextObject = document.getElementById("greet-text");
const nameInput = document.getElementById("name-input") as HTMLInputElement | null;


async function start() {
  await init();
  
  if (greetTextObject) {
    greetTextObject.textContent = setup_message();
  }
}

function greetJS() {
  if (greetTextObject) {
    let name = nameInput?.value;
    greetTextObject.textContent = greet(name ?? 'test');
  }
}

// event listen ers just need the function name, not the function name and parenthesis.
document.getElementById("greet-button")?.addEventListener("click", greetJS)

// calls the start function and makes sure that WASM is loaded.
start().catch((error: unknown) => {
  if (greetTextObject) {
    greetTextObject.textContent = 'WebAssembly failed to load. Check the browser console.';
  }
  console.error(error);
});
