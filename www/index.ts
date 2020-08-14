import { Chart } from "projections" 

const canvas = <HTMLCanvasElement>document.getElementById("canvas");

//Buttons and knobs for projection
const mapProjectionSelection = <HTMLSelectElement>document.getElementById("projection-selector");
const latLonLines = <HTMLSelectElement>document.getElementById("num-lat-lon");
const projectButton = <HTMLButtonElement>document.getElementById("project-button");

// Sliders for projections
const paramHolder = document.getElementById("param-holder");
const firstParamSlider = <HTMLDataElement> document.getElementById("first-param-slider");
const secondParamSlider = <HTMLDataElement> document.getElementById("second-param-slider");
const firstParamlabel = document.getElementById("first-param-label");
const secondParamlabel = document.getElementById("second-param-label");


//Sliders for canvas
const sizeSlider = <HTMLDataElement> document.getElementById("map-size");


//Output of those knobs
const coord = document.getElementById("coord");
const mapSizeOutput = document.getElementById("display-map-size");

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

    mapSizeOutput.innerHTML = sizeSlider.value + " units";
    sizeSlider.oninput = function() { mapSizeOutput.innerHTML = sizeSlider.value + " units"};

    projectButton.addEventListener("click", updatePlot);
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
    const selected_map = mapProjectionSelection.selectedOptions[0];
    const selected_num_points = latLonLines.selectedOptions[0];
    status.innerText = `Rendering ${selected_map.innerText}...`;
    chart = null;

    const b = Number(sizeSlider.value)/2.0;
    const bounds: Float64Array =
	Float64Array.from([b, b, -b, -b]);

    const toRadians = function (x) { return x * 3.14159 / 180; }

    const start = performance.now();
    chart = Chart.project(
	"canvas",
	selected_map.value,
	Number(selected_num_points.value),
	false,
	bounds,
	Float64Array.from([
	    toRadians(Number(firstParamSlider.value)),
	    toRadians(Number(secondParamSlider.value)),
	]),
    );

    const end = performance.now();
    status.innerText = `Rendered ${selected_map.innerText} in ${Math.ceil(end - start)}ms`;
}
