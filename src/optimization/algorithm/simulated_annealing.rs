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

        let exec_duration = start.elapsed().as_secs_f32();
        // if exec_duration as u16 % 20 == 0 {
        //     println!("経過時間: {:?}", exec_duration);
        // }
        if TIME_LIMIT < exec_duration {
            break;
        }

        // アニーリングの温度を 1 step ごとに低下させる
        temperature = cool(temperature);
    }

    (best_solution, best_makespan, best_score, iterated_num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_replaceable() {
        // 確率的な関数のため、複数のケースをテスト
        // ケース1: current < next のとき、必ず true を返す
        assert!(is_replaceable(1.0, 2.0, 1.0));
        assert!(is_replaceable(100, 200, 1.0));

        // ケース2: current > next のとき、温度が低いと false になりやすい
        // 確率的な要素があるため、単純に値を比較できないが、
        // temperature を極端に小さくすれば false になるはず
        // (注: 実際のテストでは確率的要素を考慮する必要がある)
        let result = is_replaceable(2.0, 1.0, 0.0000001);
        // 確率が極めて低いので false であることを期待
        assert!(!result);
    }

    #[test]
    fn test_is_replaceable_with_equal_values() {
        // 値が等しい場合も確率計算に入るため、温度によって結果が変わる
        let result_low_temp = is_replaceable(5.0, 5.0, 0.0000001);
        // 同じ値の場合、差分が0なので確率は exp(0) = 1 となる
        // よって常に true になるはず
        assert!(result_low_temp);
    }

    #[test]
    fn test_is_replaceable_with_extreme_values() {
        // 極端な温度のテスト
        // 温度が非常に高い場合、悪化する変更も受け入れやすくなる
        let _prob_high_temp = is_replaceable(100.0, 50.0, 1000.0);
        // 温度が高いと確率は高くなるが、乱数要素があるため確定的ではない

        // 温度が0の場合、悪化する変更は絶対に受け入れない
        let prob_zero_temp = is_replaceable(10.0, 5.0, 0.0);
        assert!(!prob_zero_temp);
    }

    #[test]
    fn test_swap_2_genes() {
        // シード値を固定して乱数を再現可能に
        let chromosome = types::Chromosome::from(vec![1, 2, 3, 4, 5]);
        let original = chromosome.clone();

        // 本来なら乱数生成器を注入できるように関数を変更すべき
        let result = swap_2_genes(&chromosome);

        // 長さは同じ
        assert_eq!(result.len(), original.len());

        // 要素の順序は変わっているはず（2つの位置が交換されている）
        // 注: 乱数によっては同じ位置が選ばれる可能性があるため、単純比較は困難
        // そのため実際のテストでは乱数生成器のモック化や注入が必要
    }

    #[test]
    fn test_swap_2_genes_identity() {
        // 長さ1の配列では交換してもそのまま
        let chromosome = types::Chromosome::from(vec![1]);
        let original = chromosome.clone();
        let result = swap_2_genes(&chromosome);
        assert_eq!(result, original);
    }

    #[test]
    fn test_swap_2_genes_preserves_elements() {
        // 要素の総和は変わらないことを確認
        let chromosome = types::Chromosome::from(vec![1, 2, 3, 4, 5]);
        let original_sum: u16 = chromosome.iter().sum();
        let result = swap_2_genes(&chromosome);
        let result_sum: u16 = result.iter().sum();
        assert_eq!(original_sum, result_sum);
    }

    #[test]
    fn test_cool() {
        let initial_temp = 1.0;
        let cooled_temp = cool(initial_temp);
        assert_eq!(cooled_temp, COOLING_RATE * initial_temp);

        let initial_temp = 10.0;
        let cooled_temp = cool(initial_temp);
        assert_eq!(cooled_temp, COOLING_RATE * initial_temp);
    }
}
