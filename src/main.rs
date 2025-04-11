mod norm_module;
use norm_module::norm::sampling_norm;

mod modules;
use modules::rust_by_example;
use modules::trait_impl_trial::{iterate_turn, Agent};

// mod optimization;
// use optimization::initializer;

// mod genetic;
// use genetic::main_algorithm;

mod utilities;
use utilities::file_readers;

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

    // initializer::init();

    // compare();

    modules::sandbox::test();

    let mut agents = vec![
        Agent::<String>::new("Hero".to_string(), 10, 5, 20),
        Agent::<String>::new("Monster".to_string(), 6, 8, 45),
    ];
    iterate_turn(agents.as_mut());

    rust_by_example::example();

    // genetic::main_algorithm::run();

    let filepath: String = utilities::file_readers::read_csv();
    println!("File path: {:?}", filepath);
}
