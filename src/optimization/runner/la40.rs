use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use crate::optimization::algorithm::simulated_annealing;
use crate::optimization::decoder::score::ScoreCalculator;
use crate::optimization::preprocess::runner::JobMaster;
use crate::optimization::types;

fn fix_random_seed(seed_num: u8) -> ChaCha8Rng {
    // 任意の固定シード（例：配列で 32bit 値を4つ）
    let seed: [u8; 32] = [seed_num; 32]; // 全部 42 にしてみる

    // RNG をシード付きで初期化
    ChaCha8Rng::from_seed(seed)
}

/// 解を1つランダムに初期化
fn initialize_chromosome(num_job: u16, num_actor: u16) -> types::Chromosome {
    // 綺麗にソートされた状態の解を生成
    let length_gene: u16 = num_job * num_actor;
    let mut chromosome: types::Chromosome =
        (0..length_gene).map(|num| num % num_job).collect();

    // 乱数生成器を使って解をシャッフル
    chromosome.shuffle(&mut fix_random_seed(42));
    println!("[INFO] Initial chromosome: {:?}", chromosome);

    chromosome
}

pub fn run(job_master: JobMaster) {
    let num_job: u16 = job_master.job_size as u16;
    let num_actor: u16 = job_master.machine_series_size as u16;
    let mut chromosome: types::Chromosome =
        initialize_chromosome(num_job, num_actor);

    let calculator: ScoreCalculator = ScoreCalculator::new(job_master);
    let (best_solution, best_makespan, best_score, iterated_num) =
        simulated_annealing::run(&mut chromosome, calculator);
    println!(
        "[INFO] best_score: {:?}, best_makespan: {:?}, iterated_num: {:?}, best_solution: {:?}",
        best_score, best_makespan, iterated_num, best_solution
    );
}
