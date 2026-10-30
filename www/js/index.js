import init, { connectwhzform, showdate, printzi, zoomtext } from "../pkg/wazi.js";

async function run() {
  const wasm = await init();
  showdate();
  connectwhzform();

  document
    .getElementById("getzi")
    .addEventListener("click", (ev) => printcar());
  document
    .getElementById("zoom")
    .addEventListener("click", (ev) => printzoom());
}

run();

function printcar() {
  printzi();
  document.getElementById("pinyin").focus();
}

function printzoom() {
  zoomtext();
  document.getElementById("pinyin").focus();
}
