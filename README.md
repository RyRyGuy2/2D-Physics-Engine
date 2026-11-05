The objective
Build a fully interactive 2D rigid-body physics sandbox without using a physics engine.

The user should be able to spawn objects, apply forces, change physical properties, and watch them interact realistically.

By the end, your program should support:
- Gravity, velocity, acceleration, and friction
- Circles and rectangles with mass
- Object-to-object collisions
- Momentum transfer and restitution (bounciness)
- Rotational motion and angular velocity
- Mouse-controlled forces and impulses
- Real-time simulation controls
- Debug overlays for velocity, forces, and collision point
  
The challenge: You must implement the physics yourself. No Rapier, Box2D, Matter.js, or other physics libraries.

## Setup

The AI-assisted setup consists only of the Rust/Wasm/TypeScript build configuration
and a connection-check message. The physics and Canvas renderer are yours to write.

Prerequisites: Rust, the wasm32-unknown-unknown target, wasm-pack, Node.js,
and Microsoft C++ Build Tools (Desktop development with C++ workload).

From PowerShell:

```powershell
cd C:\VsCode\2D-Physics-Engine\frontend
npm.cmd install
npm.cmd run dev
```

Open the local address printed by Vite. The page should display
"Rust + WebAssembly connected."

`npm.cmd run dev` rebuilds Rust before starting Vite. After editing Rust,
run `npm.cmd run build:wasm` in another terminal and refresh the browser.
`npm.cmd run build` creates a production build in frontend/dist.

Setup references: https://vite.dev/guide/ and
https://wasm-bindgen.github.io/wasm-pack/book/commands/build.html

## Rust, WebAssembly, and TypeScript tips

### What connects to what?

Rust source -> wasm-pack -> generated JavaScript wrapper + WebAssembly -> TypeScript

- Write Rust in `physics/src/`. The crate starts at `lib.rs`.
- `pub` makes a Rust item public; `#[wasm_bindgen]` exposes supported items through the generated JavaScript API.
- `wasm-pack` generates `physics/pkg/physics.js`, `physics_bg.wasm`, and TypeScript declaration files (`.d.ts`).
- Import from `physics.js`, which handles calls and data conversion between JavaScript and Wasm.
- Do not edit files in `physics/pkg/`: rebuilding replaces them.

### Calling an exported Rust function

The setup example in Rust is:

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn setup_message() -> String {
    String::from("Rust + WebAssembly connected.")
}
```

From `frontend/src/main.ts`:

```ts
import init, { setup_message } from '../../physics/pkg/physics.js';

async function start() {
  await init(); // Load and initialize Wasm before calling its exports.
  console.log(setup_message());
}

start().catch(console.error);
```

Declaring `start` does not execute it. The `start()` call does. Initialization is
asynchronous; this exported function can then be called synchronously. The wrapper
converts its Rust `String` into a JavaScript string.

### Adding or removing exports

- After adding an exported function or type, rebuild Wasm and add its name to your TypeScript import when you need it.
- After removing an export, remove any corresponding imports and calls.
- The import path stays the same when you split Rust into multiple files. Include those files in the crate's module tree using `mod`; an unreferenced `.rs` file is not compiled automatically.
- Not every Rust type can cross the Wasm boundary directly. Check the generated `.d.ts` files for the JavaScript API and accepted argument/return types.

### Daily commands

Run these from the `frontend` directory. `npm` and `npm.cmd` both work; use
`npm.cmd` if PowerShell blocks the `npm.ps1` script.

| Command | What it does |
| --- | --- |
| `npm run dev` | Rebuilds Wasm once, then starts the Vite development server. |
| `npm run build:wasm` | Rebuilds the Rust exports and generated wrappers. Run in another terminal while Vite stays open. |
| `npm run build` | Builds release Wasm, checks TypeScript, and creates `frontend/dist/`. |
| `npm run preview` | Serves an existing production build for local checking. |

After editing Rust, run `npm run build:wasm`, wait for success, then refresh the
browser. Rust is not automatically rebuilt while Vite runs. TypeScript and HTML
edits are normally picked up by Vite automatically. Plain `cargo build` does not
regenerate the JavaScript wrappers.

### Setup troubleshooting

- **Wasm returns 403:** `frontend/vite.config.ts` must allow Vite to serve `../physics/pkg`. Restart Vite after changing its configuration.
- **Missing export or stale behavior:** rebuild Wasm successfully and refresh. Check the generated `.d.ts` file to see whether the export exists.
- **Wasm is not initialized:** make sure `await init()` finishes before calling exports.
- **Page opened as a local file:** use the local URL printed by Vite rather than opening `index.html` directly.
- **Unused-variable errors during a production build:** the TypeScript configuration checks unused declarations, including code in files that are not imported by `main.ts`.

Reference: [wasm-bindgen guide](https://wasm-bindgen.github.io/wasm-bindgen/).
