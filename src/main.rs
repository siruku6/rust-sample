mod norm_module;
use norm_module::norm::sampling_norm;

mod modules;
// use modules::compare_ndarray_and_vec::compare;
use modules::rust_numpy::ndarray_sample;

// mod optimization;
// use optimization::initializer;

fn main() {
    // let mut rng = rand::rng(); // デフォルトの乱数生成器を初期化します
    // let i: i32 = rng.random(); // 整数値の乱数を生成する
    // let _f: f32 = rng.random(); // 浮動小数点数の乱数を生成する
    // println!("Random i32: {}", i);

    // 対数と指数関数の計算
    let x: f64 = 2.716;
    let y: f64 = x.ln();
    println!("ln(x): {}", y);
    println!("exp(y): {}", y.exp());

    let vec_rand: Vec<f64> = sampling_norm(10.0, 5.0, 5);
    println!("vec_rand: {vec_rand:?}");

    let a = ndarray_sample();
    println!("a:\n{a:?}");

    // initializer::init();

    // compare();

    modules::sandbox::test();
}
