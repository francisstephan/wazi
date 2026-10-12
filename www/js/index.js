import init, { connectwhzform, showdate, printzi } from "../pkg/wazi.js";

async function run() {
  const wasm = await init();
  showdate();
  connectwhzform();

  document
    .getElementById("getzi")
    .addEventListener("click", (ev) => printcar());
}

run();

function printcar() {
  printzi();
  document.getElementById("pinyin").focus();
}
