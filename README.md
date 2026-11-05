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
