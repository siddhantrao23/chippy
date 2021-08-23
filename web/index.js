import { Cpu } from "chippy";
const cpu = Cpu.new();

const ROMS = [
  "15PUZZLE",
  "BLINKY",
  "BLITZ",
  "BRIX",
  "CONNECT4",
  "GUESS",
  "HIDDEN",
  "IBM",
  "INVADERS",
  "KALEID",
  "MAZE",
  "MERLIN",
  "MISSILE",
  "PONG",
  "PONG2",
  "PUZZLE",
  "SYZYGY",
  "TANK",
  "TETRIS",
  "TICTAC",
  "UFO",
  "VBRIX",
  "VERS",
  "WIPEOFF"
];

const WIDTH = 64;
const HEIGHT = 32;

const canvas = document.getElementById("chip8-canvas");

const ctx = canvas.getContext("2d");
ctx.fillStyle = "black";
ctx.fillRect(0, 0, WIDTH, HEIGHT);

ROMS.forEach(rom => {
    $("#roms").append(`<option value='${rom}'>${rom}</option>`);
});

let running = false;
const run_btn = document.getElementById("run");
run_btn.addEventListener("click", () => {
    if(running) {
        run_btn.innerHTML = "Start";
    } else {
        run_btn.innerHTML = "Stop";
    }
    running = !running;
})

const renderLoop = () => {
    cpu.execute_cycle();

    drawGrid();
    drawCells();

    requestAnimationFrame(renderLoop);
};
