import * as wasm from 'kernel-calculation';
const wasmtime = performance.now();
wasm.fibonacci(40);
console.log(performance.now() - wasmtime);

function fibonacci(i) {
  if (i === 1 || i === 2) {
    return 1;
  }
  return fibonacci(i - 1) + fibonacci(i - 2);
}

const a = performance.now();
fibonacci(40);
console.log(performance.now() - a);
