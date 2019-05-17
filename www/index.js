import { PoissonDisk } from "poisson-disk";

const RADIUS = 5;
const NUM_SAMPLES = 5;

const canvas = document.getElementById('viz');
const GRID_WIDTH = canvas.width;
const GRID_HEIGHT = canvas.height;

const disk = PoissonDisk.new(GRID_WIDTH, GRID_HEIGHT, RADIUS, NUM_SAMPLES);
const ctx = canvas.getContext('2d');

if (window.devicePixelRatio > 1) {
    var canvasWidth = canvas.width;
    var canvasHeight = canvas.height;
    canvas.width = canvasWidth * window.devicePixelRatio;
    canvas.height = canvasHeight * window.devicePixelRatio;
    canvas.style.width = canvasWidth;
    canvas.style.height = canvasHeight;
    ctx.scale(window.devicePixelRatio, window.devicePixelRatio);
}

const getIndex = (row, column) => {
    return row * GRID_WIDTH + column;
};

const drawSamples = (drawRadius) => {
    const num_points = disk.num_points();

    for (let i = 0; i < num_points; i++) {
        const point = disk.point_at_idx(i);

        ctx.beginPath();
        ctx.fillStyle = '#000000';
        ctx.fillRect(point[0], point[1], 1, 1);
        ctx.stroke();

        if (drawRadius) {
            ctx.beginPath();
            ctx.ellipse(
                point[0], point[1],
                RADIUS, RADIUS,
                0, 0, 2 * Math.PI
            );
            ctx.stroke();
        }
    }
}

const SAMPLES_PER_FRAME = 10;
const renderLoop = () => {
    drawSamples(false);
    for (let i = 0; i < SAMPLES_PER_FRAME; i++) {
        disk.tick();
    }
    requestAnimationFrame(renderLoop);
};

renderLoop();