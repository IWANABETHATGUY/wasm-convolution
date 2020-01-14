mod utils;

use wasm_bindgen::prelude::*;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// function convertJsFilter(data, width, height, kernel, divisor) {
//     const kw = kernel[0].length;
//     const kh = kernel.length;
//     const half = Math.floor(kw / 2);
//     for (let y = 1; y < height; y++) {
//       for (let x = 1; x < width; x++) {
//         let px = (y * width + x) * 4;
//         let r = 0,
//           g = 0,
//           b = 0;
//         for (let ky = 0; ky < kh; ky++) {
//           for (let kx = 0; kx < kw; kx++) {
//             let kernelValue = kernel[ky][kx];
//             let cpx = ((y + ky - half) * width + (x + kx - half)) * 4;
//             r += data[cpx] * kernelValue;
//             g += data[cpx + 1] * kernelValue;
//             b += data[cpx + 2] * kernelValue;
//           }
//         }
//         let finalR = r / divisor;
//         let finalG = g / divisor;
//         let finalB = b / divisor;
//         data[px] = finalR > 255 ? 255 : finalR < 0 ? 0 : finalR;
//         data[px + 1] = finalG > 255 ? 255 : finalG < 0 ? 0 : finalG;
//         data[px + 2] = finalB > 255 ? 255 : finalB < 0 ? 0 : finalB;
//       }
//     }
//     return data;
//   }
#[wasm_bindgen]
pub fn convert_js_filter(
    data: &mut [u8],
    width: usize,
    height: usize,
    kernel: &[i32],
    divisor: usize,
    k_width: usize,
    k_height: usize,
) -> usize {
    let half = (k_width / 2) as usize;
    let mut px: usize;
    for y in half..height - half {
        for x in half..width - half {
            px = (y * width + x) * 4;
            let mut r = 0;
            let mut g = 0;
            let mut b = 0;
            for i in 0..k_height {
                for j in 0..k_width {
                    let kernel_value = kernel[(i * k_width + j) as usize];
                    let cpx = ((y + i - half) * width + (x + j - half)) * 4;
                    r += data[cpx] as i32 * kernel_value;
                    g += data[cpx + 1] as i32 * kernel_value;
                    b += data[cpx + 2] as i32 * kernel_value;
                }
            }
            data[px] = match r / divisor as i32 {
                a @ 0..=255 => a,
                256..=std::i32::MAX => 255,
                _ => 0,
            } as u8;
            data[px + 1] = match g / divisor as i32 {
                a @ 0..=255 => a,
                256..=std::i32::MAX => 255,
                _ => 0,
            } as u8;
            data[px + 2] = match b / divisor as i32 {
                a @ 0..=255 => a,
                256..=std::i32::MAX => 255,
                _ => 0,
            } as u8;
        }
    }
    1
}
