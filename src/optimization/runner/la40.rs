use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use crate::optimization::decoder::score::calc_makespan;
use crate::optimization::preprocess::runner::JobMaster;

fn fix_random_seed(seed_num: u8) -> ChaCha8Rng {
    // 任意の固定シード（例：配列で 32bit 値を4つ）
    let seed: [u8; 32] = [seed_num; 32]; // 全部 42 にしてみる

    // RNG をシード付きで初期化
    ChaCha8Rng::from_seed(seed)
}

/// 解を1つランダムに初期化
fn initialize_chromosome(num_job: u16, num_actor: u16) -> Vec<u16> {
    // 綺麗にソートされた状態の解を生成
    let length_gene: u16 = num_job * num_actor;
    let mut chromosome: Vec<u16> =
        (0..length_gene).map(|num| num % num_job).collect();

    // 乱数生成器を使って解をシャッフル
    // let mut rng = rand::rng();
    chromosome.shuffle(&mut fix_random_seed(42));
    println!("[INFO] Initial chromosome: {:?}", chromosome);

    chromosome
}

pub fn run(job_master: JobMaster) {
    let num_job: u16 = job_master.job_size as u16;
    let num_actor: u16 = job_master.machine_series_size as u16;
    let chromosome: Vec<u16> = initialize_chromosome(num_job, num_actor);

    let make_span: u16 = calc_makespan(&chromosome, &job_master);
    println!("[INFO] makespan: {:?}", make_span);
}
