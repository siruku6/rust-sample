use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;

/// コマンドライン引数の最初の値を取得します。
/// 引数がない場合はエラーを返します。
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    // コマンドライン引数を取得
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none ...")),
        Some(file_path) => Ok(file_path),
    }
}

/// CSVファイルをパースして、行ごとのデータを文字列ベクターとして返します。
/// パースに失敗した場合はエラーを返します。
fn csv_parse(file: File) -> Result<Vec<String>, Box<dyn Error>> {
    let mut row_list = Vec::new();

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);
    for result in rdr.records() {
        let record: String = result?.deserialize(None)?;
        // println!("{:?}", record);
        row_list.push(record.clone());
    }
    Ok(row_list)
}

/// 指定されたパスのファイルを開きます。
/// ファイルが開けない場合はエラーを返します。
fn fopen(path: &OsString) -> Result<File, Box<dyn Error>> {
    let file: File = match File::open(path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file {:?}: {}", path, err);
            process::exit(1);
        }
    };
    Ok(file)
}

/// コマンドライン引数で指定されたCSVファイルを読み込み、
/// 各行を出力します。ファイルパスを文字列として返します。
///
/// # Examples
///
/// $ cargo run data/input/la40_tailored.txt
///
pub fn read_csv() -> String {
    let file_path = match get_first_arg() {
        Ok(file_path) => file_path,
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };

    let file: File = fopen(&file_path).unwrap();
    let row_list = match csv_parse(file) {
        Ok(row_list) => {
            println!("Successfully parsed CSV: {:?}", file_path);
            row_list
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };

    for row in row_list {
        println!("Row: {:?}", row);
    }

    file_path.into_string().unwrap()
}
