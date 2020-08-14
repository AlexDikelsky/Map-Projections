import { Chart } from "projections" 

const canvas = <HTMLCanvasElement>document.getElementById("canvas");
const coord = document.getElementById("coord");
const plotType = <HTMLSelectElement>document.getElementById("plot-type");
const status = document.getElementById("status");

let chart = null;

/** Main entry point */
export function main() {
    setupUI();
    setupCanvas();
}

/** Add event listeners. */
function setupUI() {
    status.innerText = "WebAssembly loaded!";
    plotType.addEventListener("change", updatePlot);
    window.addEventListener("resize", setupCanvas);
    window.addEventListener("mousemove", onMouseMove);
}

/** Setup canvas to properly handle high DPI and redraw current plot. */
function setupCanvas() {
    const dpr = window.devicePixelRatio || 1;
    const aspectRatio = canvas.width / canvas.height;
    const parentnode = <HTMLElement>canvas.parentNode;
    const size = Math.min(canvas.width, parentnode.offsetWidth);
    canvas.style.width = size + "px";
    canvas.style.height = size / aspectRatio + "px";
    canvas.width = size * dpr;
    canvas.height = size / aspectRatio * dpr;
    canvas.getContext("2d").scale(dpr, dpr);
    updatePlot();
}

/** Update displayed coordinates. */
function onMouseMove(event) {
    if (chart) {
        const point = chart.coord(event.offsetX, event.offsetY);
        coord.innerText = (point)
            ? `(${point.x.toFixed(3)}, ${point.y.toFixed(3)})`
            : "Mouse pointer is out of range";
    }
}

/** Redraw currently selected plot. */
function updatePlot() {
    const selected = plotType.selectedOptions[0];
    status.innerText = `Rendering ${selected.innerText}...`;
    chart = null;

    const bounds: Float64Array =
	Float64Array.from([2.0, 2.0, -2.0, -2.0]);

    const start = performance.now();
    chart = (selected.value == "mandelbrot")
        ? Chart.project("canvas", "mapnamehere", Number(4), false, bounds)
        : Chart.project("canvas", "mapnamehere", Number(selected.value), false, bounds);
    const end = performance.now();
    status.innerText = `Rendered ${selected.innerText} in ${Math.ceil(end - start)}ms`;
}
