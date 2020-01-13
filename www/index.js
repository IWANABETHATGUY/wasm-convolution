import { memory } from 'kernel-calculation/kernel_calculation_bg';

const canvas = document.querySelector('.canvas-scenery');
const video = document.querySelector('.video');
const fpsNum = document.querySelector('.fps-num');

const kernel = [
  [-1, -1, -1],
  [-1, 8, -1],
  [-1, -1, -1],
];
const divisor = 4;

const ctx = canvas.getContext('2d');
let clientWidth, clientHeight, pixels;

let lastTime = performance.now();

function getFpsNum() {
  let cur = performance.now();
  let duration = cur - lastTime;
  lastTime = cur;
  fpsNum.textContent = Math.round(1000 / duration);
  draw();
  window.requestAnimationFrame(getFpsNum);
}
function convertJsFilter(data, width, height, kernel, divisor) {
  
  const kw = kernel[0].length;
  const kh = kernel.length;

  const half = Math.floor(kw / 2);
  for (let y = 1; y < height; y++) {
    for (let x = 1; x < width; x++) {
      let px = (y * width + x) * 4;
      let r = 0,
        g = 0,
        b = 0;
      for (let ky = 0; ky < kh; ky++) {
        for (let kx = 0; kx < kw; kx++) {
          let kernelValue = kernel[ky][kx];
          let cpx = ((y + ky - half) * width + (x + kx - half)) * 4;

          r += data[cpx] * kernelValue;
          g += data[cpx + 1] * kernelValue;
          b += data[cpx + 2] * kernelValue;
        }
      }
      let finalR = r / divisor;
      let finalG = g / divisor;
      let finalB = b / divisor;
      data[px] = finalR > 255 ? 255 : finalR < 0 ? 0 : finalR;
      data[px + 1] = finalG > 255 ? 255 : finalG < 0 ? 0 : finalG;
      data[px + 2] = finalB > 255 ? 255 : finalB < 0 ? 0 : finalB;
    }
  }
  return data;
}

function filterJs(pixels, clientWidth, clientHeight) {
  return convertJsFilter(pixels.data, clientWidth, clientHeight, kernel, divisor);
}
function draw() {
  ctx.drawImage(video, 0, 0);
  pixels = ctx.getImageData(0, 0, video.videoWidth, video.videoHeight);
  pixels.data.set(filterJs(pixels, clientWidth, clientHeight));
  ctx.putImageData(pixels, 0, 0)
}

// draw();
video.onloadeddata = function() {
  canvas.setAttribute('width', video.videoWidth);
  canvas.setAttribute('height', video.videoHeight);

  clientHeight = canvas.clientHeight;
  clientWidth = canvas.clientWidth;

  getFpsNum();
};
