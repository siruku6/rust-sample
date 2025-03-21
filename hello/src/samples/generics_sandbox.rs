pub mod default {
    use std::fmt::Display;

    pub fn init() {
        println!("[INFO] --------- generics_sandbox ---------");

        square_easy_sample();

        impl_generics_func();

        generic_val_test();

        impl_with_generics();

        constraint_ok();

        // constraint_ng1();

        // constraint_ng2();
    }

    /* -------------------------
    Currently in trial
    ------------------------- */
    #[derive(Debug)]
    struct A;

    #[derive(Debug)]
    struct Single(A);

    // ジェネリクスを持つ構造体。Tは任意の型を表す。
    #[derive(Debug)]
    struct SGen<T>(T);

    impl<T> SGen<T> {
        // ジェネリクスを使用したメソッド。
        // `T`は構造体で定義された型パラメータ。
        // `where T: std::fmt::Debug` は `T` が `Debug` トレイトを実装している必要があることを示す。
        fn generic(&self, some: &T)
        where
            T: std::fmt::Debug,
        {
            println!("{:?}", some);
        }
    }

    fn impl_generics_func() {
        let _s = Single(A); // `Single`構造体に`A`を渡してインスタンス化。
        let _char: SGen<char> = SGen('a'); // `SGen`構造体に`char`型を渡してインスタンス化。
        let _t = SGen(A); // `SGen`構造体に`A`型を渡してインスタンス化。
        let _i32 = SGen(6); // `SGen`構造体に`i32`型を渡してインスタンス化。

        // 上で生成した変数を出力
        println!("{:?}", _s);
        println!("{:?}", _char);
        println!("{:?}", _t);
        println!("{:?}", _i32);

        // `SGen`構造体のインスタンスを生成し、`generic`メソッドを呼び出す。
        let instance = SGen('a');
        instance.generic(&'b'); // `'b'を渡して出力。
    }

    /* -------------------------
    impl for override
    ------------------------- */
    struct S; // Concrete type `S`
    struct GenericVal<T> {
        // Generic type `GenericVal`
        variable: T,
    }

    // `<T>` Must precede the type to remain generic
    // ジェネリック型のまま扱うには`<T>`が先に来る必要がある。
    impl<T> GenericVal<T> {
        fn var(&self) -> &T {
            &self.variable
        }
    }

    fn generic_val_test() {
        let x: GenericVal<i32> = GenericVal { variable: 3 };
        let y: GenericVal<f32> = GenericVal { variable: 10.0 };

        println!("{:?}", x.var());
        println!("{:?}", y.var());
    }

    /* -------------------------
    impl without generics
    ------------------------- */
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    fn square_easy_sample() {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area()
        );
    }

    /* -------------------------
    impl with generics
    ------------------------- */

    #[derive(Debug)]
    struct Empty;
    struct Null;

    trait DoubleDrop<T> {
        // `self`に加えてもう一つジェネリック型を受け取り、
        // 何もしないメソッドのシグネチャを定義
        fn double_drop(self, _: T);
    }

    // `U`を`self`として、`T`をもう一つの引数として受け取る`DoubleDrop<T>`
    // を実装する。`U`,`T`はいずれもジェネリック型
    impl<T, U> DoubleDrop<T> for U {
        // このメソッドは2つの引数の所有権を取り、メモリ上から開放する。
        fn double_drop(self, _: T) {}
    }

    fn impl_with_generics() {
        let empty = Empty;
        let null = Null;
        let rect: Rectangle = Rectangle {
            width: 30,
            height: 50,
        };
        let empty2 = Empty;

        // Deallocate `empty` and `null`.
        // `empty`と`null`を開放
        null.double_drop(empty);
        // empty.double_drop(null);  // 交換可能
        // impl for <GenType> だと、あらゆる struct に対して impl される
        rect.double_drop(empty2);

        // println!("empty: {:?}", empty);
        //empty;
        //null;
        // ^ TODO: Try uncommenting these lines.
        // ^ TODO: これらの行をアンコメントしてみましょう。
    }

    /* -------------------------
    impl with constraints
    ------------------------- */
    // ジェネリック型を持つ構造体
    struct Wrapper<T> {
        value: T,
    }

    // `Wrapper`の実装
    impl<T> Wrapper<T>
    where
        T: Display, // `T`は`Display`トレイトを実装している必要がある
    {
        fn display_value(&self) {
            println!("[Constraint OK] {}", self.value);
        }
    }

    fn constraint_ok() {
        // i32はDisplayを実装している
        let wrapped_int = Wrapper { value: 42 };
        // &strもDisplayを実装している
        let wrapped_str = Wrapper {
            value: "Hello, Rust!",
        };
        wrapped_int.display_value(); // 問題なく動作
        wrapped_str.display_value(); // 問題なく動作
    }

    // fn constraint_ng1() {
    //     // Vec<T>はDisplayを実装していない
    //     let wrapped_vec = Wrapper {
    //         value: vec![1, 2, 3],
    //     };
    //     wrapped_vec.display_value(); // コンパイルエラー
    // }

    // fn constraint_ng2() {
    //     // Vec<T>はDisplayを実装していない
    //     struct NonDisplayType; // `Display`を実装していない型

    //     let wrapped_non_display = Wrapper {
    //         value: NonDisplayType,
    //     };

    //     wrapped_non_display.display_value(); // コンパイルエラー
    // }
}
