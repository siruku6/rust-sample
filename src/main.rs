mod norm_module;
use norm_module::norm::sampling_norm;

// mod modules;
// use modules::trait_impl_trial::{iterate_turn, Agent};

mod optimization;
use optimization::preprocess;
use optimization::preprocess::runner::JobMaster;
use optimization::runner::la40;
// use optimization::initializer;

mod utilities;
use utilities::file_readers;

fn main() {
    // 対数と指数関数の計算
    let x: f64 = 2.716;
    let y: f64 = x.ln();
    println!("ln(x): {}", y);
    println!("exp(y): {}", y.exp());

    let vec_rand: Vec<f64> = sampling_norm(10.0, 5.0, 5);
    println!("vec_rand: {vec_rand:?}");

    // initializer::init();

    // let mut agents = vec![
    //     Agent::<String>::new("Hero".to_string(), 10, 5, 20),
    //     Agent::<String>::new("Monster".to_string(), 6, 8, 45),
    // ];
    // iterate_turn(agents.as_mut());

    /* --------------------------------
    最適化処理のサンプル実装
    -------------------------------- */
    let (file_path, delimiter, delimiter_str) = file_readers::parse_args();
    let (header, row_list) =
        file_readers::read_csv(file_path, delimiter, delimiter_str);
    let job_master: JobMaster = preprocess::runner::run(header, row_list);

    la40::run(job_master);
}
