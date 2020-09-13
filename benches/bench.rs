#![feature(test)]

extern crate test;
use kernel_calculation;
use rand::Rng;
#[bench]
fn universe_ticks(b: &mut test::Bencher) {
    let mut rng = rand::thread_rng();
    let width = 800;
    let height = 600;
    let mut vec = Vec::with_capacity(width * height * 4);
    for i in 0..width * height * 4 {
        vec.push(rng.gen::<u8>());
    }
    let kernel: Vec<i32> = vec![1, 1, 1, 1, -5, 1, 1, 1];
    // kernel_calculation::convert_js_filter();
    b.iter(|| {
        kernel_calculation::convert_js_filter(&mut vec, width, height, &kernel, 4, 3, 3);
    });
}