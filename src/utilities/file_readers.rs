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
    let mut header: Option<StringRecord> = None;
    let mut row_list: Vec<StringRecord> = Vec::new();

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

// // 既存のcsv_parse関数をこの新しい関数のラッパーにする
// fn csv_parse(file: File) -> Result<Vec<StringRecord>, Box<dyn Error>> {
//     parse_delimited_file(file, b',').map(|(_, rows)| rows) // カンマ区切り
// }

// // タブ区切りファイル用の新しい関数
// fn tsv_parse(file: File) -> Result<Vec<StringRecord>, Box<dyn Error>> {
//     parse_delimited_file(file, b'\t').map(|(_, rows)| rows) // タブ区切り
// }

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

/// コマンドライン引数を解析するための構造体
struct ArgParser {
    file_path: OsString,
    delimiter: u8,
    delimiter_str: String,
}
impl ArgParser {
    /// 新しいArgParserを作成します。
    /// 引数はコマンドライン引数から取得します。
    fn new() -> Self {
        let file_path = get_first_arg().unwrap();
        let delimiter_str = get_second_arg()
            .unwrap_or_else(|_| OsString::from("comma"))
            .into_string()
            .unwrap_or_else(|_| "comma".to_string());

        let delimiter = match delimiter_str.to_lowercase().as_str() {
            "tab" => b'\t',
            "comma" => b',',
            "semicolon" => b';',
            "pipe" => b'|',
            "space" => b' ',
            _ => b',',
        };

        let delimiter_str = match delimiter {
            b'\t' => "tab".to_string(),
            b',' => "comma".to_string(),
            b';' => "semicolon".to_string(),
            b'|' => "pipe".to_string(),
            b' ' => "space".to_string(),
            _ => "comma".to_string(),
        };

        Self {
            file_path,
            delimiter,
            delimiter_str,
        }
    }
}

pub fn parse_args() -> (OsString, u8, String) {
    // コマンドライン引数を解析
    let arg_parser = ArgParser::new();
    let file_path = arg_parser.file_path;
    let delimiter: u8 = arg_parser.delimiter;
    let delimiter_str: String = arg_parser.delimiter_str;

    (file_path, delimiter, delimiter_str)
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
pub fn read_csv(
    file_path: OsString,
    delimiter: u8,
    delimiter_str: String,
) -> (Option<StringRecord>, Vec<StringRecord>) {
    // ファイルを開く
    let file: File = fopen(&file_path).unwrap();
    let (header, row_list) = match parse_delimited_file(file, delimiter) {
        Ok(result) => {
            let (header, row_list): (Option<StringRecord>, Vec<StringRecord>) =
                result;
            println!(
                "Successfully {:?} is parsed with {:?}",
                file_path, delimiter_str
            );
            (header, row_list)
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };

    // file_path.into_string().unwrap()
    (header, row_list)
}
