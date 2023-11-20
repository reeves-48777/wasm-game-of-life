import { memory } from "wasm-game-of-life/wasm_game_of_life_bg.wasm";

import { Universe } from "wasm-game-of-life";

const CELL_SIZE = 15; // pixels
const GRID_COLOR = "#DDDDDD";
const DEAD_COLOR = "#EEEEEE";
const ALIVE_COLOR = "#333333";
const LEFT_MOUSE_BUTTON = 0;
const BORDER_SIZE = 1;


const universe = Universe.new();
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

    const bitIsSet = (n, arr) => {
        const byte = Math.floor(n / 8);
        const mask = 1 << (n % 8);
        return (arr[byte] & mask) === mask;
    }

    const drawCells = () => {
        const cellsPtr = universe.cells();
        const cells = new Uint8Array(memory.buffer, cellsPtr, width * height / 8);

        ctx.beginPath();

        for (let row = 0; row < height; row++) {
            for (let col = 0; col < width; col++) {
                const idx = getIndex(row, col);

                ctx.fillStyle = bitIsSet(idx,cells)
                    ? ALIVE_COLOR
                    : DEAD_COLOR;

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

    const isPaused = () => {
        return animationId === null;
    }

    const playPauseButton = document.getElementById("play-pause");

    const play = () => {
        playPauseButton.textContent = "⏸️";
        renderLoop();
    };

    const pause = () => {
        playPauseButton.textContent = "▶️";
        cancelAnimationFrame(animationId);
        animationId = null;
    };

    playPauseButton.addEventListener("click", _ => {
        if (isPaused()) {
            play();
        } else {
            pause();
        }
    });

    canvas.addEventListener("click", e => {
        const boundingRect = canvas.getBoundingClientRect();
        
        const scaleX = canvas.width / boundingRect.width;
        const scaleY = canvas.height / boundingRect.height;

        const canvasLeft = (e.clientX - boundingRect.left) * scaleX;
        const canvasTop = (e.clientY - boundingRect.top) * scaleY;

        const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
        const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width -1);

        universe.toggle_cell(row, col);

        drawGrid();
        drawCells();
    })


    let animationId = null;
    const renderLoop = () => {
        // debugger;
        universe.tick();
        draw();
        animationId = requestAnimationFrame(renderLoop);
    };

    draw();
    play();
}
