fn main() {
    println!("hello {0}, i am {1}. {1}, hello {0}", "a", 123);
    println!("{}12313", 1);

    // as can named arguments
    println!(
        "name={name}, age={age}, {name}+{age}",
        name = "devhg",
        age = 1231,
    );

    // special formatting can be specified after a ":"
    println!(
        "{} of {:b} people known binary, the other half doesn't",
        1, 2
    );

    println!("{number:>width$}", number = 1, width = 6);
    println!("{number:>0width$}", number = 1, width = 6);

    #[derive(Debug)]
    struct Structure(i32);

    #[derive(Debug)]
    struct Deep(Structure);

    println!("This struct `{:?}` won't print", Structure(1));
    println!("This struct `{:#?}` print", Deep(Structure(222)));

    println!("{:?} month in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name. {2:?}",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    let name = "devhg";
    let age = 21;
    let peter = Person { name, age };
    println!("{:#?}", peter);
}
