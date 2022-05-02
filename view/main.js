import init, { greet, run_simon } from "simon_game";

init().then(() => {
  console.log("init wasm-pack");
  // greet("from vite!");
  run_simon();
});
