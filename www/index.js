import { memory } from "wasm-game-of-life/wasm_game_of_life_bg.wasm";

import { Universe, Cell, CommonSpaceships } from "wasm-game-of-life";

const CELL_SIZE = 15; // pixels
const GRID_COLOR = "#DDDDDD";
const DEAD_COLOR = "#EEEEEE";
const ALIVE_COLOR = "#333333";
const LEFT_MOUSE_BUTTON = 0;
const BORDER_SIZE = 1;


const universe = Universe.new();
universe.add_spaceship(CommonSpaceships.Heavyweight, 0, 32)
universe.add_spaceship(CommonSpaceships.Lightweight, 32, 12)
universe.add_spaceship(CommonSpaceships.Glider, 0, 0);
const width = universe.width();
const height = universe.height();

// On init le canvas pour qu'il ait la place pour toutes les cellules et un bord de 1px;
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

if (canvas.getContext) {
    const ctx = canvas.getContext("2d");

    let scale = 1;
    const zoomSens = 0.1;
    let offsetX = 0, offsetY = 0;

    const drawGrid = () => {
        ctx.beginPath();
        ctx.strokeStyle = GRID_COLOR;


        // lignes verticales
        for (let v = 0; v <= width; v++) {
            ctx.moveTo(v * (CELL_SIZE + 1) + BORDER_SIZE / 2, 0);
            ctx.lineTo(v * (CELL_SIZE + 1) + BORDER_SIZE / 2, (CELL_SIZE + 1) * height + BORDER_SIZE / 2);
        }

        // lignes horizontale
        for (let h = 0; h <= height; h++) {
            ctx.moveTo(0, h * (CELL_SIZE + 1) + BORDER_SIZE / 2);
            ctx.lineTo((CELL_SIZE + 1) * width + BORDER_SIZE / 2, h * (CELL_SIZE + 1) + BORDER_SIZE / 2);
        }

        ctx.stroke();
    };

    const getIndex = (row, col) => {
        return row * width + col;
    };

    const drawCells = () => {
        const cellsPtr = universe.cells();
        const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

        ctx.beginPath();

        for (let row = 0; row < height; row++) {
            for (let col = 0; col < width; col++) {
                const idx = getIndex(row, col);

                ctx.fillStyle = cells[idx] === Cell.Dead
                    ? DEAD_COLOR
                    : ALIVE_COLOR;

                ctx.fillRect(
                    col * (CELL_SIZE + BORDER_SIZE) + 1,
                    row * (CELL_SIZE + BORDER_SIZE) + 1,
                    CELL_SIZE,
                    CELL_SIZE
                );
            }
        }
        ctx.stroke();
    };

    const draw = () => {
        ctx.save();
        ctx.clearRect(0,0, canvas.width, canvas.height);
        ctx.translate(offsetX, offsetY);
        ctx.scale(scale, scale);

        // main draw here
        drawGrid();
        drawCells();

        ctx.restore();
    };

    const getMousePosition = (canvas, evt) => {
        var rect = canvas.getBoundingClientRect();
        return {
            x: evt.clientX - rect.left,
            y: evt.clientY - rect.top
        };
    };

    const zoom = (event) => {
        const mousePosition = getMousePosition(canvas, event);

        const oldScale = scale;
        if (event.deltaY < 0) {
            // zoom in
            scale *= (1 + zoomSens);
        } else {
            // zoom out
            scale /= (1 + zoomSens);
        }

        offsetX -= (mousePosition.x - offsetX) * (scale - oldScale) / oldScale;
        offsetY -= (mousePosition.y - offsetY) * (scale - oldScale) / oldScale;

        event.preventDefault();
        draw();
    };

    canvas.onwheel = zoom;

    const startDrag = (event) => {
        if (event.button === LEFT_MOUSE_BUTTON) {
            const startX = event.clientX - offsetX;
            const startY = event.clientY - offsetY;

            const drag = (event) => {
                offsetX = event.clientX - startX;
                offsetY = event.clientY  - startY;
                draw();
            }

            const stopDrag = () => {
                canvas.removeEventListener('mousemove', drag);
                window.removeEventListener('mouseup', stopDrag);
            }

            canvas.addEventListener('mousemove', drag);
            window.addEventListener('mouseup', stopDrag);
        }
    };
    
    canvas.onmousedown = startDrag;


    const renderLoop = () => {
        universe.tick();
        draw();
        requestAnimationFrame(renderLoop);
    };

    draw();
    requestAnimationFrame(renderLoop);
}
