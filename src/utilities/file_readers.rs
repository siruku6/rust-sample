use csv::StringRecord;
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

fn get_second_arg() -> Result<OsString, Box<dyn Error>> {
    // コマンドライン引数を取得
    match env::args_os().nth(2) {
        None => Err(From::from("expected 2nd argument, but got none ...")),
        Some(delimiter) => Ok(delimiter),
    }
}

/// ファイルをパースして、1行目とそれ以降の行を別々に返します。
/// 区切り文字を指定できます（例: カンマはb',', タブはb'\t'）
/// パースに失敗した場合はエラーを返します。
fn parse_delimited_file(
    file: File,
    delimiter: u8,
) -> Result<(Option<StringRecord>, Vec<StringRecord>), Box<dyn Error>> {
    let mut header = None;
    let mut row_list = Vec::new();

    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .has_headers(false)
        .delimiter(delimiter)
        .from_reader(file);

    let mut iter = rdr.records();

    // 最初の行を取得
    if let Some(result) = iter.next() {
        header = Some(result?);
    }

    // 残りの行を処理
    for result in iter {
        let record = result?;
        row_list.push(record);
    }

    Ok((header, row_list))
}

// 既存のcsv_parse関数をこの新しい関数のラッパーにする
fn csv_parse(file: File) -> Result<Vec<StringRecord>, Box<dyn Error>> {
    parse_delimited_file(file, b',').map(|(_, rows)| rows) // カンマ区切り
}

// タブ区切りファイル用の新しい関数
fn tsv_parse(file: File) -> Result<Vec<StringRecord>, Box<dyn Error>> {
    parse_delimited_file(file, b'\t').map(|(_, rows)| rows) // タブ区切り
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

/// コマンドライン引数で指定されたファイルを読み込み、
/// 各行を出力します。ファイルパスを文字列として返します。
///
/// # Examples
///
/// $ cargo run data/input/la40_tailored.txt tab
/// $ cargo run data/input/la40_tailored.txt comma
/// $ cargo run data/input/la40_tailored.txt semicolon
///
pub fn read_csv() -> String {
    let file_path = match get_first_arg() {
        Ok(file_path) => file_path,
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };

    // 2番目の引数から区切り文字を取得
    let delimiter_str = match get_second_arg() {
        Ok(delimiter) => delimiter
            .into_string()
            .unwrap_or_else(|_| "comma".to_string()),
        Err(_) => {
            println!("No delimiter specified, using comma as default.");
            "comma".to_string()
        }
    };

    // 区切り文字の文字列を対応するバイトに変換
    let delimiter = match delimiter_str.to_lowercase().as_str() {
        "tab" => b'\t',
        "comma" => b',',
        "semicolon" => b';',
        "pipe" => b'|',
        "space" => b' ',
        _ => {
            println!(
                "Unknown delimiter: {}, using comma as default.",
                delimiter_str
            );
            b','
        }
    };

    let file: File = fopen(&file_path).unwrap();
    let (header, row_list) = match parse_delimited_file(file, delimiter) {
        Ok(result) => {
            let (header, row_list) = result;
            println!(
                "Successfully parsed file with '{}' delimiter: {:?}",
                delimiter_str, file_path
            );
            (header, row_list)
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };

    // ヘッダーを表示
    println!("Header: {:?}", header);
    // 各行を表示
    for row in row_list {
        println!("Row: {:?}", row);
    }

    file_path.into_string().unwrap()
}
