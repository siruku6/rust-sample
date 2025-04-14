use std::collections::HashMap;

use csv::StringRecord;

pub struct JobMaster {
    pub job_size: usize,
    pub machine_series_size: usize,
    pub exec_times: Vec<Vec<i16>>,
    pub actor_sequences: Vec<Vec<i16>>,
}

/// CSVファイルのヘッダーから job 数と machine_series 数を取得
fn extract_size(header: Option<StringRecord>, target_name: String) -> usize {
    let mapping: HashMap<String, usize> = HashMap::from([
        ("job_size".to_string(), 0),
        ("machine_series_size".to_string(), 1),
    ]);

    let index: usize = *mapping
        .get(&target_name)
        .unwrap_or_else(|| panic!("{:?} not found", target_name));

    let size: usize = header
        .unwrap()
        .get(index)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0); // パースに失敗した場合は0をデフォルト値とする

    size
}

pub fn run(header: Option<StringRecord>, rows: Vec<StringRecord>) -> JobMaster {
    /* -------------------------------
    rows を Vec<Vec<i16>> に変換

    - header から job 数を取得
    - rows から実行時間と operation ごとの actor_id のを取得
    ------------------------------- */

    // headerから job 数を取得
    let job_size: usize = extract_size(header.clone(), "job_size".to_string());
    let machine_series_size: usize =
        extract_size(header, "machine_series_size".to_string());

    // rows を Vec<Vec<i16>> に変換
    let mut exec_times: Vec<Vec<i16>> = Vec::new();
    let mut actor_id_sequences: Vec<Vec<i16>> = Vec::new();
    for (i, row) in rows.iter().enumerate() {
        let mut row_data: Vec<i16> = Vec::new();
        for field in row.iter() {
            if let Ok(value) = field.parse::<i16>() {
                row_data.push(value);
            }
        }

        // job 内各 operation の所要時間の整理
        if i < job_size {
            exec_times.push(row_data);
        // job 内各 operation のを実行可能なマシンの整理
        } else {
            actor_id_sequences.push(row_data);
        }
    }

    let job_master: JobMaster = JobMaster {
        job_size,
        machine_series_size,
        exec_times: exec_times.clone(),
        actor_sequences: actor_id_sequences.clone(),
    };

    job_master
}
