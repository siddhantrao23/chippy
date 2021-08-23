import { Cpu } from "chippy";
const cpu = Cpu.new();

const WIDTH = 64;
const HEIGHT = 32;

const canvas = document.getElementById("chip8-canvas");

const ctx = canvas.getContext("2d");
ctx.fillStyle = "black";
ctx.fillRect(0, 0, WIDTH, HEIGHT);

const renderLoop = () => {
    cpu.execute_cycle();

    drawGrid();
    drawCells();

    requestAnimationFrame(renderLoop);
};
