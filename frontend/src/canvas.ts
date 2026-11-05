// should be the controller that actually makes the changes to the canvas element.
// gets it directly

// renderer.ts will generate the actual things to put and canvas.ts will visualize it

// need a function call getCanvasValues() returns an

// canvas defautls to 300x150 px

const canvas = document.getElementById("canvas") as HTMLCanvasElement;
const ctx = canvas.getContext("2d");

function test() {
    if (ctx == null || ctx == undefined) {
        return
    }
    ctx.fillStyle = "green";

    // Add a rectangle at (10, 10) with size 100x100 pixels
    ctx.fillRect(10, 10, 100, 100);
}

test();