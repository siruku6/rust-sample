use crate::optimization::decoder::score::ScoreCalculator;
use crate::optimization::types;
use rand::Rng;
use std::ops::Sub;
use std::time::Instant;

static COOLING_RATE: f64 = 0.9;
static STEPS: u32 = 2_000_000;
static TIME_LIMIT: f32 = 600.0; // seconds

/// 現在のスコアと次のスコアを比較して、確率的に次の値を選択する
/// current: 現在のスコア
/// next: 次のスコア
/// temperature: 温度パラメータ
///     低いほど、次のスコアを選択しやすくなる
fn is_replaceable<T>(current: T, next: T, temperature: f64) -> bool
where
    T: PartialOrd + Sub<Output = T> + Into<f64> + Copy,
{
    if current < next {
        return true;
    }
    let diff: f64 = (next - current).into();
    let prob: f64 = (diff / temperature).exp();
    if rand::random::<f64>() < prob {
        return true;
    }
    false
}

/// 染色体（解）の中からランダムに2つの遺伝子を選択して、その2つの位置を交換
fn swap_2_genes(chromosome: &types::Chromosome) -> types::Chromosome {
    let num_genes: usize = chromosome.len();

    // TODO: 乱数生成を統一 seed に従わせる
    let mut rng = rand::rng();
    let first_idx: usize = rng.random_range(0..num_genes) as usize;
    let second_idx: usize = rng.random_range(0..num_genes) as usize;
    let mut new_chromo: types::Chromosome = chromosome.clone();
    new_chromo.swap(first_idx, second_idx);
    new_chromo.to_owned()
}

fn cool(temperature: f64) -> f64 {
    COOLING_RATE * temperature
}

pub fn run(
    chromosome: &mut types::Chromosome,
    calculator: ScoreCalculator,
) -> (types::Chromosome, u16, f64, u32) {
    let start: Instant = Instant::now();
    let mut temperature: f64 = 1.0;

    let mut current_solution: types::Chromosome = chromosome.clone();
    let current_makespan: u16 =
        calculator.calc_makespan(current_solution.clone());
    let mut current_score: f64 = calculator.makespan_to_score(current_makespan);

    let mut best_solution: types::Chromosome = chromosome.clone();
    let mut best_makespan: u16 = current_makespan;
    let mut best_score: f64 = current_score;

    let mut iterated_num: u32 = 0;
    for _step in 0..STEPS {
        iterated_num += 1;
        let tmp_chromosome: types::Chromosome = swap_2_genes(&current_solution);
        let tmp_makespan: u16 =
            calculator.calc_makespan(tmp_chromosome.clone());
        let tmp_score: f64 = calculator.makespan_to_score(tmp_makespan);

        // annealing アルゴリズムに基づき、解を更新できるか判定
        let replacable: bool =
            is_replaceable(current_score, tmp_score, temperature);

        if replacable {
            current_score = tmp_score;
            current_solution = tmp_chromosome.clone();
        }

        if best_score < tmp_score {
            best_score = tmp_score;
            best_makespan = tmp_makespan;
            best_solution = tmp_chromosome;
            println!(
                "step: {:?}, score: {:?}, makespan: {:?}",
                _step, best_score, best_makespan
            );
        }

        let duration = start.elapsed().as_secs_f32();
        // if duration as u16 % 20 == 0 {
        //     println!("処理時間: {:?}", duration);
        // }
        if TIME_LIMIT < duration {
            break;
        }

        // アニーリングの温度を 1 step ごとに低下させる
        temperature = cool(temperature);
    }

    (best_solution, best_makespan, best_score, iterated_num)
}
