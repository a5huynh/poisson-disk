import { PoissonDisk } from "poisson-disk";
import { memory } from "poisson-disk/poisson_disk_bg";

const CELL_SIZE = 5;
const GRID_WIDTH = 256;
const GRID_HEIGHT = 96;
const RADIUS = 10;

const disk = PoissonDisk.new(GRID_WIDTH, GRID_HEIGHT, RADIUS);

const canvas = document.getElementById('viz');
canvas.height = (CELL_SIZE + 1) * GRID_HEIGHT + 1;
canvas.width = (CELL_SIZE + 1) * GRID_WIDTH + 1;
const ctx = canvas.getContext('2d');

const getIndex = (row, column) => {
    return row * GRID_WIDTH + column;
};

const drawCells = () => {
    const cellsPtr = disk.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, GRID_WIDTH * GRID_HEIGHT);

    ctx.beginPath();

    for (let row = 0; row < GRID_HEIGHT; row++) {
        for (let col = 0; col < GRID_WIDTH; col++) {
            const idx = getIndex(row, col);
            if (cells[idx] === 1) {
                ctx.fillStyle = '#000000';
            } else if (cells[idx] === 2) {
                ctx.fillStyle = '#FF0000';
            } else {
                ctx.fillStyle = '#FFFFFF';
            }

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
}

const drawRadius = () => {
    const cellsPtr = disk.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, GRID_WIDTH * GRID_HEIGHT);

    for (let row = 0; row < GRID_HEIGHT; row++) {
        for (let col = 0; col < GRID_WIDTH; col++) {
            const idx = getIndex(row, col);
            if (cells[idx] === 1 || cells[idx] === 2) {
                let cell_radius = CELL_SIZE * RADIUS;
                ctx.beginPath();
                let x = col * (CELL_SIZE + 1) + CELL_SIZE / 2 + 1;
                let y = row * (CELL_SIZE + 1) + CELL_SIZE / 2 + 1;
                ctx.ellipse(
                    x, y,
                    cell_radius, cell_radius,
                    0, 0, 2 * Math.PI
                );
                ctx.stroke();
            }
        }
    }
}


const renderLoop = () => {
    drawCells();
    drawRadius();
    if (disk.tick()) {
        requestAnimationFrame(renderLoop);
    } else {
        console.log('done!');
    }
};

renderLoop();