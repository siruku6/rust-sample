use crate::optimization::preprocess::runner::JobMaster;
use crate::optimization::types;
use std::collections::HashMap;

pub struct ScoreCalculator {
    pub job_master: JobMaster,
    done_operations_count: HashMap<u16, u8>,
}

// 各 job の operation 進捗
fn init_counter(job_size: u16) -> HashMap<u16, u8> {
    let mut done_operations_count: HashMap<u16, u8> = HashMap::new();
    for job in 0..job_size {
        done_operations_count.insert(job, 0);
    }
    done_operations_count
}

impl ScoreCalculator {
    pub fn new(job_master: JobMaster) -> Self {
        ScoreCalculator {
            job_master: job_master.clone(),
            done_operations_count: init_counter(job_master.job_size),
        }
    }

    pub fn makespan_to_score(&self, makespan: u16) -> f64 {
        1.0 / makespan as f64
    }

    /// 該当 job の operation 進捗を更新
    fn count_up(&mut self, job_id: &u16) {
        self.done_operations_count
            .insert(*job_id, self.done_operations_count[job_id] + 1);
    }

    fn reset(&mut self) {
        self.done_operations_count.clear();
        self.done_operations_count = init_counter(self.job_master.job_size);
    }

    pub fn calc_makespan(&mut self, chromosome: types::Chromosome) -> u16 {
        /* ----------------------------------------------------------------
        // 下準備 - カウンターの初期化
        ---------------------------------------------------------------- */
        // 各 job 内実行済み operation の経過時間
        let mut elapsed_job_time_map: HashMap<u16, u16> = HashMap::new();
        for job in 0..self.job_master.job_size {
            elapsed_job_time_map.insert(job, 0);
        }
        // 各 actor の経過時間
        let mut elapsed_actor_time_map: HashMap<u16, u16> = HashMap::new();
        for actor_id in 0..self.job_master.machine_series_size {
            elapsed_actor_time_map.insert(actor_id, 0);
        }

        /* ----------------------------------------------------------------
        // makespan の計算
        ---------------------------------------------------------------- */
        // gene は job_id
        for job_id in chromosome.iter() {
            // ---------------------- 値の取得 ----------------------
            // 次に実行する operation の番号
            let operation_no: u8 = self.done_operations_count[job_id];

            // 次の operation の実行時間を取得
            let operation_time: u16 = self.job_master.exec_times
                [*job_id as usize][operation_no as usize];

            // 次の operation の actor_id を取得
            let actor_id: u16 = self.job_master.actor_sequences
                [*job_id as usize][operation_no as usize];

            // ---------------------- 値の更新 ----------------------
            // 該当 actor の経過時間を更新
            elapsed_actor_time_map.insert(
                actor_id,
                elapsed_actor_time_map[&actor_id] + operation_time,
            );
            // 該当 job の経過時間を更新
            elapsed_job_time_map
                .insert(*job_id, elapsed_job_time_map[job_id] + operation_time);

            // 該当 job の経過時間と actor の経過時間のうち大きい方の時間で、小さい方の経過時間を更新
            if elapsed_actor_time_map[&actor_id] < elapsed_job_time_map[job_id]
            {
                elapsed_actor_time_map
                    .insert(actor_id, elapsed_job_time_map[job_id]);
            } else {
                elapsed_job_time_map
                    .insert(*job_id, elapsed_actor_time_map[&actor_id]);
            }

            self.count_up(job_id);
        }

        self.reset();

        // 各 job の経過時間のうち最大値を取得
        let makespan: u16 =
            elapsed_job_time_map.values().max().unwrap().to_owned();

        makespan
    }
}
