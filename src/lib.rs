#![feature(clamp)]

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
) {
    let half = (k_width / 2) as usize;
    let mut px: usize;
    let mut r;
    let mut g;
    let mut b;
    let divisor = divisor as i32;
    let mut cpx;
    let mut kernel_value;
    for y in 1..(height - half) {
        for x in 1..(width - half) {
            r = 0;
            g = 0;
            b = 0;
            for i in 0..k_height {
                let i_k_width = i * k_width;
                let y_extract_half_mul_i = (y + i - half) * width;
                let x_extract_half = x - half;
                for j in 0..k_width {
                    kernel_value = unsafe { *kernel.get_unchecked(i_k_width + j) };
                    cpx = (y_extract_half_mul_i + (x_extract_half + j)) * 4;
                    unsafe {
                        r += *data.get_unchecked(cpx) as i32 * kernel_value;
                        g += *data.get_unchecked(cpx + 1) as i32 * kernel_value;
                        b += *data.get_unchecked(cpx + 2) as i32 * kernel_value;
                    }
                }
            }
            px = (y * width + x) * 4;
            unsafe {
                *data.get_unchecked_mut(px) = (r / divisor).clamp(0, 255) as u8;
                *data.get_unchecked_mut(px + 1) = (g / divisor).clamp(0, 255) as u8;
                *data.get_unchecked_mut(px + 2) = (b / divisor).clamp(0, 255) as u8;
            }
        }
    }
}

#[wasm_bindgen]
pub fn fibonacci(v: i32) -> i32 {
    match v {
        1 | 2 => 1,
        _ => fibonacci(v - 1) + fibonacci(v - 2),
    }
}
