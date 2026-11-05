import init, { setup_message } from '../../physics/pkg/physics.js';

const app = document.querySelector<HTMLDivElement>('#app')!;
app.textContent = 'Loading WebAssembly...';

async function start() {
  await init();
  app.textContent = setup_message();
}

start().catch((error: unknown) => {
  app.textContent = 'WebAssembly failed to load. Check the browser console.';
  console.error(error);
});
