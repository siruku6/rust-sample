use std::collections::HashMap;

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

pub fn type_sample() {
    println!("[INFO] --------- Hello, type_sample! ---------");

    /* -------------------------
    String -> &str
    ------------------------- */
    let s1: String = String::from("hello");
    let s2: &str = &s1;
    let s3: String = s2.to_string(); // &str -> String
    println!("s1: {}, s2: {}, s3: {}", s1, s2, s3);

    /* -------------------------
    tuple
    ------------------------- */
    let mut t = (1, 2, "2", 3.0);
    println!("t: {:?}", t);
    t.0 = 2;
    t.2 = "3";
    println!("t: {:?}", t);

    /* -------------------------
    array
    ------------------------- */
    let mut ar: [i32; 5] = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [0; 5];
    ar[1] = b[1];
    ar[2] = b[2];
    println!("ar: {:?}", &ar[1..4]);

    /* -------------------------
    struct
    ------------------------- */
    // struct Point {
    //     name: String,
    //     x: i32,
    //     y: f32,
    // }
    // let p = Point {
    //     name: String::from("point1"),
    //     x: 1,
    //     y: 2.2,
    // };
    // let Point {x: i, y: j, name: name} = p;

    struct Example(i32, f32, String);

    let Example(i, j, name) = Example(1, 2.2, String::from("point2"));

    println!("struct: {}, {}, {}", i, j, name);

    /* -------------------------
    enum
    ------------------------- */
    enum Color {
        Red,
        // Green,
        // Blue,
        TransparentBlue { opacity: f32 },
    }

    let a = Color::Red;
    let c = Color::TransparentBlue { opacity: 0.5 };

    // パターンマッチングを使う
    match c {
        Color::TransparentBlue { opacity } => {
            println!("The opacity is: {}", opacity); // 正しい方法
        }
        _ => {
            println!("This color doesn't have opacity");
        }
    }

    match a {
        Color::TransparentBlue { opacity } => {
            println!("The opacity is: {}", opacity); // ない
        }
        _ => {
            println!("This color doesn't have opacity");
        }
    }

    /* -------------------------
    Option

    as_ref() は Option<T> を Option<&T> に変換する
    ------------------------- */
    let text: Option<String> = Some("Hello, world!".to_string());
    let text_ref: Option<&String> = text.as_ref();
    println!("[Option] can print text: {text:?}");
    println!("[Option] can print text: {text_ref:?}");

    // First, cast `Option<String>` to `Option<&String>` with `as_ref`,
    // then consume *that* with `map`, leaving `text` on the stack.
    let text_length: Option<usize> = text.as_ref().map(|s| s.len());
    println!("text_length: {text_length:?}");
    println!("still can print text: {text:?}");

    // https://doc.rust-lang.org/std/option/enum.Option.html#method.as_deref
    // Option<T> (or &Option<T>) to Option<&T::Target>
    println!("Some('hey'): {:?}", type_of(&Some("hey"))); // "core::option::Option<&str>"
    let x: Option<String> = Some("hey".to_owned());
    println!("x: {:?}", type_of(&x)); // "core::option::Option<alloc::string::String>"
    println!("x.as_deref(): {:?}", type_of(&x.as_deref())); // "core::option::Option<&str>"

    // match との組み合わせ
    fn divide(a: i32, b: i32) -> Option<i32> {
        if b == 0 {
            None // ゼロ除算はエラーとして処理
        } else {
            Some(a / b) // 正常な結果を返す
        }
    }
    // 問題ない例
    let result = divide(10, 2);
    match result {
        Some(x) => println!("Result: {}", x),
        None => println!("Error: division by zero"),
    }

    // ゼロ除算の例
    let result = divide(10, 0);
    match result {
        Some(x) => println!("Result: {}", x),
        None => println!("Error: division by zero"),
    }

    /* -------------------------
    Result
    ------------------------- */
    fn divide_error(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err("division by zero".to_string())
        } else {
            Ok(a / b)
        }
    }

    // 結果を活用する
    let result = divide_error(19, 2);
    match result {
        Ok(x) => println!("Result1: {}", x), // Okの場合: 19 / 2 は 9 になる模様
        Err(e) => println!("Error1: {}", e),
    }

    let result = divide_error(10, 0);
    match result {
        Ok(x) => println!("Result2: {}", x),
        Err(e) => println!("Error2: {}", e),
    }

    // if letを使う
    if let Ok(x) = divide_error(19, 2) {
        println!("Result3-1: {}", x);
    }

    if let Ok(x) = divide_error(1, 0) {
        println!("Result3-2: {}", x);
    }
    if let Err(e) = divide_error(1, 0) {
        println!("Error3: {}", e);
    }

    // unwrap_or を使う
    let result = divide_error(19, 2).unwrap_or(-1);
    println!("Result4-1: {}", result);
    let result = divide_error(1, 0).unwrap_or(-1);
    println!("Result4-2: {}", result);

    // unwrap_or_else を使う
    // result? を使う
    fn error_handling(result: Result<i32, String>) -> Result<i32, String> {
        let content = result?; // エラーの場合はここで return result; が呼ばれる
        Ok(content)
    }
    let result = divide_error(19, 2);
    let code = error_handling(result);
    println!("Result5-1: {:?}", code);

    let result = divide_error(1, 0);
    let code = error_handling(result);
    println!("Result5-2: {:?}", code);

    /* -------------------------
    Vec の使い方
    ------------------------- */
    let mut v_blank: Vec<i32> = vec![];
    println!("v_blank: {:?}", v_blank);
    v_blank.push(1);
    v_blank.push(2);

    let v_ser = vec![1, 2, 3, 4, 5];
    // let v_zeros = vec![0; 5];

    println!("for loop for Vec");
    for element in &v_ser {
        println!("v_ser: {}", element);
    }

    /* -------------------------
    Box の特徴
    - コンパイル時にサイズがわからない型を格納できる
    - 大きなサイズの型の値を渡す際にデータの中身をコピーせずにポインタを渡すことができる
    - 共通の trait を実装した様々な型を画一的にポインタで扱うことができる
    ------------------------- */
    fn print_5_components(s: [u8; 5]) {
        // 5つの要素の変数しか受け取れない
        println!("{:?}", s);
    }

    let byte_array = [b'h', b'e', b'l', b'l', b'o'];
    print_5_components(byte_array);

    // fn print(s: [u8]) {
    //     // 要素数未定だとエラー
    //     println!("{:?}", s);
    // }

    fn print(s: Box<[u8]>) {
        // Box なら要素数未定でもOK
        println!("{:?}", s);
    }

    let byte_array = [b'h', b'e', b'l', b'l', b'o'];
    // Box::new で Box<[u8]> 型に変換すれば要素数未定でもOK
    print(Box::new(byte_array));

    /* -------------------------
    Range
    ------------------------- */
    let r = 1..5;
    for i in r {
        println!("i: {}", i);
    }

    let r = 1..=5;
    for j in r {
        println!("j: {}", j);
    }

    /* -------------------------
    Iterator
    ------------------------- */
    struct Counter {
        current: usize,
        end: usize,
    }

    impl Iterator for Counter {
        type Item = usize;

        fn next(&mut self) -> Option<usize> {
            self.current += 1;

            if self.current - 1 < self.end {
                Some(self.current - 1)
            } else {
                None
            }
        }
    }

    let counter = Counter {
        current: 0,
        end: 10,
    };
    for i in counter {
        println!("i: {}", i);
    }

    /* -------------------------
    HashMap
    ------------------------- */
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // output hashmap
    println!("{:?}", scores);

    // & をつけないと、所有権が移動してしまう
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    for (key, value) in scores {
        println!("{key}: {value}");
    }

    // キーが存在しない場合のみ挿入
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{scores:?}");

    // use HashMap as a counter
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");

    /* -------------------------
    mut
    ------------------------- */
    let immutable_x = 10;
    let mut x = 5;

    x += immutable_x;
    println!("x: {}", x);

    /* -------------------------
    型指定
    ------------------------- */
    let x: u64 = 10;
    let y: f64 = 10.0;
    let z: bool = true;
    let a: char = 'a';
    // let err: u32 = -1; // error: cannot apply unary operator `-` to type `u32`

    println!("x: {}, y: {}, z: {}, a: {}", x, y, z, a);

    const MAX_POINTS: u32 = 100_000;
    // MAX_POINTS = 100_001; // error: cannot assign to `MAX_POINTS`, as it is a constant
    println!("MAX_POINTS: {}", MAX_POINTS);
}
