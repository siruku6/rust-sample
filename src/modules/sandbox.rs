fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

pub fn test() {
    let x: Option<String> = Some("hey".to_owned());
    assert_eq!(x.as_deref(), Some("hey"));
    // assert_eq!(x, Some("hey"));

    println!("sandbox test");
    println!("'hey': {:?}", type_of(&"hey"));
    println!("String::from('hey'): {:?}", type_of(&String::from("hey")));
    println!("String::from('hey'): {:?}", type_of(&String::from("hey")));
    println!("Some('hey'): {:?}", type_of(&Some("hey")));
    let x: Option<String> = Some("hey".to_owned());

    println!("x: {:?}", type_of(&x));
    println!("x.as_deref(): {:?}", type_of(&x.as_deref()));
}
