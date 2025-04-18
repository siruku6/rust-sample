use std::collections::HashMap;

use crate::optimization::preprocess::runner::JobMaster;
use crate::optimization::types;

pub struct ScoreCalculator {
    pub job_master: JobMaster,
    done_operations_count: HashMap<u16, u8>, // 各 job の operation 進捗
    elapsed_job_time_map: HashMap<u16, u16>,
    elapsed_actor_time_map: HashMap<u16, u16>,
}

impl ScoreCalculator {
    pub fn new(job_master: JobMaster) -> Self {
        ScoreCalculator {
            job_master: job_master.clone(),
            done_operations_count: HashMap::new(),
            elapsed_job_time_map: HashMap::new(),
            elapsed_actor_time_map: HashMap::new(),
        }
    }

    pub fn makespan_to_score(&self, makespan: u16) -> f64 {
        1.0 / makespan as f64
    }

    fn init_counters(&mut self) {
        /* ----------------------------------------------------------------
        // 下準備 - カウンターの初期化
        ---------------------------------------------------------------- */
        let mut done_operations_count: HashMap<u16, u8> = HashMap::new();
        for job in 0..self.job_master.job_size {
            done_operations_count.insert(job, 0);
        }
        self.done_operations_count = done_operations_count;

        // 各 job 内実行済み operation の経過時間
        self.elapsed_job_time_map = HashMap::new();
        for job in 0..self.job_master.job_size {
            self.elapsed_job_time_map.insert(job, 0);
        }
        // 各 actor の経過時間
        self.elapsed_actor_time_map = HashMap::new();
        for actor_id in 0..self.job_master.machine_series_size {
            self.elapsed_actor_time_map.insert(actor_id, 0);
        }
    }

    fn update_elapsed_time(
        &mut self,
        actor_id: u16,
        job_id: u16,
        operation_time: u16,
    ) {
        // 該当 actor の経過時間を更新
        self.elapsed_actor_time_map.insert(
            actor_id,
            self.elapsed_actor_time_map[&actor_id] + operation_time,
        );
        // 該当 job の経過時間を更新
        self.elapsed_job_time_map.insert(
            job_id,
            self.elapsed_job_time_map[&job_id] + operation_time,
        );

        // 該当 job の経過時間と actor の経過時間のうち大きい方の時間で、小さい方の経過時間を更新
        if self.elapsed_actor_time_map[&actor_id]
            < self.elapsed_job_time_map[&job_id]
        {
            self.elapsed_actor_time_map
                .insert(actor_id, self.elapsed_job_time_map[&job_id]);
        } else {
            self.elapsed_job_time_map
                .insert(job_id, self.elapsed_actor_time_map[&actor_id]);
        }
    }

    /// 該当 job の operation 進捗を更新
    fn count_up(&mut self, job_id: &u16) {
        self.done_operations_count
            .insert(*job_id, self.done_operations_count[job_id] + 1);
    }

    pub fn calc_makespan(&mut self, chromosome: types::Chromosome) -> u16 {
        self.init_counters();

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
            self.update_elapsed_time(actor_id, *job_id, operation_time);

            self.count_up(job_id);
        }

        // 各 job の経過時間のうち最大値を取得
        let makespan: u16 =
            self.elapsed_job_time_map.values().max().unwrap().to_owned();

        makespan
    }
}
