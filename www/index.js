import { memory } from "wasm-game-of-life/wasm_game_of_life_bg.wasm";
import { CommonSpaceships, Universe } from "wasm-game-of-life";

import {fps} from "./js/fps"

const CELL_SIZE = 1; // pixels
const BORDER_SIZE = 1; // pixel
const BORDER_OFFSET = 1; // pixel

const GRID_COLOR = "#a0a0a0";
const DEAD_COLOR = "#e0e0e0";
const ALIVE_COLOR = "#3E3E3E";
const RMB = 2;

const bitStorageSize = 8;

const gridHeight = 512; // cells
const gridWidth = 512; // cells

const universe = Universe.new();

if (gridWidth !== null) {
    universe.set_width(gridWidth);
}
if (gridHeight !== null) {
    universe.set_height(gridHeight);
}
universe.random_cells();
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

    const getIndex = (row, col) => {
        return row * width + col;
    };

    const drawCell = (row, col) => {
        ctx.fillRect(
            col * (CELL_SIZE + BORDER_SIZE) + BORDER_OFFSET,
            row * (CELL_SIZE + BORDER_SIZE) + BORDER_OFFSET,
            CELL_SIZE,
            CELL_SIZE
        );
    };

    const bitIsSet = (bitIndex, cellsArray) => {
        const byteIndex = Math.floor(bitIndex / bitStorageSize);
        const mask = 1 << (bitIndex % bitStorageSize);
        return (cellsArray[byteIndex] & mask) !== 0;
    };

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

    const drawCells = () => {
        const cellsPtr = universe.cells();
        const cells = new Uint8Array(memory.buffer, cellsPtr, width * height / bitStorageSize);

        ctx.beginPath();

        // alive cells
        ctx.fillStyle = ALIVE_COLOR;
        for (let row = 0; row < height; row++) {
            for (let col = 0; col < width; col++) {
                const idx = getIndex(row, col);

                if (!bitIsSet(idx, cells)) {
                    continue;
                }
                drawCell(row, col);

            }
        }

        // dead cells
        ctx.fillStyle = DEAD_COLOR;
        for (let row = 0; row < height; row++) {
            for (let col = 0; col < width; col++) {
                const idx = getIndex(row, col);

                if (bitIsSet(idx, cells)) {
                    continue;
                }
                drawCell(row, col);

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
        if (event.button === RMB) {
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

    // sets all the cells to dead
    const deadCellsButton = document.getElementById("dead");
    deadCellsButton.addEventListener("click", _ => {
        universe.dead_cells();
    });

    // sets all the cells to random
    const randomCellsButton = document.getElementById("random");
    randomCellsButton.addEventListener("click", _ => {
        universe.random_cells();
    });

    // play/pause button
    const playPauseButton = document.getElementById("play-pause");

    const isPaused = () => {
        return animationId === null;
    }

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
        const rect = canvas.getBoundingClientRect();
        const style = window.getComputedStyle(canvas);

        const padding = parseInt(style.padding, 10);
        const borderWidth = parseInt(style.borderWidth, 10);

        const scaleX = canvas.width / rect.width;
        const scaleY = canvas.height / rect.height;

        const x = (e.clientX - rect.left - padding - borderWidth) * scaleX;
        const y = (e.clientY - rect.top - padding - borderWidth) * scaleY;

        const clickOffset = BORDER_SIZE / 2;

        const col = Math.floor((x - clickOffset) / (CELL_SIZE + BORDER_SIZE));
        const row = Math.floor((y - clickOffset) / (CELL_SIZE + BORDER_SIZE));

        if (row >= 0 && row < height && col >= 0 && col < width) {
            if (e.ctrlKey) {
                universe.add_spaceship(CommonSpaceships.Glider, col-1, row-1);
            } else if (e.shiftKey) {
                universe.add_spaceship(CommonSpaceships.Heavyweight, col-3, row-2);
            } else {
                universe.toggle_cell(row, col);
                            }
    
            drawGrid();
            drawCells();
        }
        // debug avec un cercle rouge pour voir la ou le click est détecté
        // ctx.strokeStyle = "#FF0000";
        // ctx.beginPath();
        // ctx.arc(x, y, 5, 0, Math.PI * 2);
        // ctx.stroke();

    });

    canvas.addEventListener("contextmenu", e => e.preventDefault());

    let ticksPerFrame = document.getElementById("ticks-per-frame");

    let animationId = null;

    const renderLoop = () => {
        fps.render();
        for (let i = 0; i < ticksPerFrame.value; i++) {
            universe.tick();
        }
        draw();
        animationId = requestAnimationFrame(renderLoop);
    };

    // document.addEventListener('keypress', (e) => {
    //     if (e.code === "Space") {
    //         e.preventDefault();
    //         universe.tick();
    //         drawDelta;
    //     }
    // })

    draw();
    play();
}
