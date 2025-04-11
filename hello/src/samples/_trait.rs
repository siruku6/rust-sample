pub mod trait_sample {
    trait Greet {
        fn greet(&self);

        fn greet_twice(&self) {
            self.greet();
            self.greet();
        }

        fn shout(&self) {
            println!("HELLO!");
        }
    }

    struct Child;
    struct Adult;

    impl Greet for Child {
        fn greet(&self) {
            println!("Hi! :D");
        }
    }

    impl Greet for Adult {
        fn greet(&self) {
            println!("Hello.");
        }
    }

    pub fn trait_sample_func() {
        println!("[INFO] --------- trait_sample ---------");

        let c = Child {};
        let a = Adult {};

        c.greet();
        c.greet_twice();
        c.shout();

        a.greet();
        a.greet_twice();
        a.shout();

        let people_vec: Vec<Box<dyn Greet>> = vec![Box::new(c), Box::new(a)];
        for person in people_vec {
            person.greet();
        }
    }
}
