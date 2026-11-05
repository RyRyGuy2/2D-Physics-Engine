const canvas = document.getElementById("canvas") as HTMLCanvasElement;
const ctx = canvas.getContext("2d");

class Square {
    x: number;
    y: number;
    length: number;
    rotation: number;

    constructor (
        x: number,
        y: number,
        length: number,
        rotation: number
    ) {
        this.x = x;
        this.y = y;
        this.length = length;
        this.rotation = rotation;
    }
}

class Circle {
    x: number;
    y: number;
    radius: number;

    constructor (
        x: number,
        y: number,
        radius: number
    ) {
        this.x = x;
        this.y = y;
        this.radius = radius;
    }
}

function test() {
    if (ctx == null || ctx == undefined) {
        return
    }

    drawSquare(100, 100, 50, 0, "green");
    drawCirlce(200, 100, 50, "blue")
}

// rotation should be in radians
function drawSquare(x: number, y: number, length: number, rotation: number, color: string = "green") {
    if (ctx == null) return;

    let xVals: number[] = [];
    let yVals: number[] = [];

    let halfWidth = length / 2;
    let radius = Math.sqrt(2 * halfWidth ** 2);

    for (let i = 0; i < 4; i++) {
        let angle = rotation + Math.PI / 4 + i * Math.PI / 2;

        let xVal = x + radius * Math.cos(angle);
        let yVal = y + radius * Math.sin(angle);

        xVals.push(xVal);
        yVals.push(yVal);
    }

    ctx.beginPath();
    ctx.moveTo(xVals[0], yVals[0]);

    for (let i = 1; i < 4; i++) {
        ctx.lineTo(xVals[i], yVals[i]);
    }

    ctx.closePath();

    ctx.fillStyle = color;
    ctx.fill();

    ctx.lineWidth = 4;
    ctx.strokeStyle = "black";
    ctx.stroke();
}



function drawCirlce(x: number, y: number, radius: number, color: string = "blue") {
    if (ctx == null || ctx == undefined) return;

    ctx.beginPath();
    ctx.arc(x, y, radius, 0, 2 * Math.PI)
    ctx.stroke();
    ctx.fillStyle = color;
    ctx.fill();
    ctx.lineWidth = 4;
    ctx.strokeStyle = "black";
    ctx.stroke();
}

function clearCanvas() {
    ctx?.clearRect(0, 0, canvas.width, canvas.height);
}
test();