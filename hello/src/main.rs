use clap::{arg, App, ArgMatches};

/* -------------------------
Use handmade modules
------------------------- */
use hello::sample_module::hello;
use hello::samples::_trait::trait_sample::trait_sample_func;
use hello::samples::control::control_sample;
use hello::samples::generics_sandbox::default::init;
use hello::samples::types::type_sample;

fn main() {
    /* -------------------------
    コマンドライン引数
    ------------------------- */
    let matches: ArgMatches = App::new("MyApp")
        .version("0.1.0")
        .author("Author")
        .about("Does awesome things")
        .arg(arg!([FILE] "input").help("input file").required(false))
        .arg(
            arg!(-v --verbose ... "Sets the level of verbosity")
                .help("level of verbosity")
                .required(false),
        )
        .get_matches();

    match matches.value_of("FILE") {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified"),
    }
    let verbose = matches.is_present("verbose");
    println!("Is verbosity specified?: {}", verbose);

    /* -------------------------
    Sample for types
    ------------------------- */
    // type_sample();

    /* -------------------------
    Control like if, while, and so on ...
    ------------------------- */
    // control_sample();

    /* -------------------------
    impl
    構造体にメソッドを追加し、クラスのように使うことができる
    ------------------------- */
    struct Person {
        name: String,
        age: u8,
    }

    impl Person {
        fn say_hello(&self) -> &Self {
            println!("Hello, my name is {}.", self.name);
            self
        }

        fn say_age(&self) -> &Self {
            println!("I am {} years old.", self.age);
            self
        }

        fn new(name: &str, age: u8) -> Self {
            Person {
                name: String::from(name),
                age,
            }
        }
    }

    let p: Person = Person {
        name: String::from("Alice"),
        age: 20,
    };
    p.say_hello();
    p.say_age();
    p.say_hello().say_age();

    let p: Person = Person::new("Bob", 30);
    p.say_hello().say_age();

    /* -------------------------
    trait

    - impl trait for a struct
    ------------------------- */
    // trait_sample_func();

    /* -------------------------
    generics

    敢えて型を指定しないことで、他の型の引数が入力されても対応できる
    ------------------------- */
    fn print_type<T>(_: T) {
        println!("Type: {}", std::any::type_name::<T>());
    }
    print_type(1);

    fn make_tuple<T, U>(t: T, u: U) -> (T, U) {
        (t, u)
    }
    let t1: (i32, i32) = make_tuple(1, 2);
    println!("t: {:?}", t1);

    /* -------------------------
    generics sandbox
    ------------------------- */
    init();

    /* -------------------------
    Use module
    ------------------------- */
    hello();

    /* -------------------------
    slice string
    ------------------------- */
    let s = String::from("hello");
    let mut slice = &s[0..4];
    println!("slice String: {}", slice);

    let s: &str = "hello";
    slice = &s[0..2];
    println!("slice &str: {}", slice);

    /* -------------------------
    collect iterator
    ------------------------- */
    let vec_sample: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // let vec_sample2: Vec<i32> = vec_sample.iter().copied().collect();
    let vec_sample2: Vec<i32> = vec_sample.to_vec();
    println!("length {:?}", vec_sample.len());
    println!("length {:?}", vec_sample.len());
    println!("length {:?}", vec_sample2.len());
}
