pub fn control_sample() {
    // このファイルの役割を println! マクロで表示
    println!("[INFO] --------- Hello, control! ---------");

    /* -------------------------
    if
    ------------------------- */
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 6;
    let result = if number % 4 == 0 {
        "divisible by 4"
    } else if number % 3 == 0 {
        "divisible by 3"
    } else if number % 2 == 0 {
        "divisible by 2"
    } else {
        "not divisible by 4, 3, or 2"
    };
    println!("The number {} is {}", number, result);

    /* -------------------------
    loop
    ------------------------- */
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
        println!("counter: {}", counter);
    };
    println!("The result is {}", result);

    /* -------------------------
    while
    ------------------------- */
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("number is decreased into: {}", number);

    /* -------------------------
    for
    ------------------------- */
    // let count: i32;
    for count in (1..4).rev() {
        println!("{}!", count);
    }

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    /* -------------------------
    ループ系ブロックからの離脱
    ------------------------- */
    let mut num: i32 = 0;
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            num += 1;
            if num == 5 {
                println!("reach 5");
                break 'inner;
            } else if num == 10 {
                println!("reach 10");
                break 'outer;
            }
        }
        println!("This point will not be reached twice.");
    }

    /* -------------------------
    match
    ------------------------- */
    let number = 13;
    match number {
        1 => println!("One!"),
        2 => println!("Two!"),
        3 => println!("Three!"),
        4 => println!("Four!"),
        5 => println!("Five!"),
        _ => println!("Something else!"),
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    let c = Coin::Quarter;
    match c {
        Coin::Penny => println!("Penny!"),
        Coin::Nickel => println!("Nickel!"),
        // Coin::Dime => println!("Dime!"), // 必須
        Coin::Dime => println!("Dime!"),
        Coin::Quarter => println!("Quarter!"),
    }

    // match 結果を変数に格納
    let c = Coin::Quarter;
    let value = match c {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    };
    println!("The value of the coin is: {}", value);

    let result: Result<i32, String> = Ok(200);
    let code = match result {
        Ok(content) => content,
        Err(e) => {
            println!("Error: {}", e);
            -1
        }
    };
    println!("code: {}", code);

    let result: Result<i32, String> = Err("error".to_string());
    let _code = match result {
        Ok(content) => content,
        Err(e) => {
            println!("Error: {}", e);
            -1
        }
    };
}
