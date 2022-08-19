use std::fmt;

pub fn main_1_2() {
    println!("\n====== main_1_2 ======");
    println!("Hello World");
    println!("I'm a Rustacean!!");

    let align: usize = 10;
    let number: i64 = 7086163;
    println!("{number:0>align$}");

    // Rust even checks to make sure the correct number of arguments are
    // used.
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // FIXME ^ Add the missing argument: "James"

    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types do not implement fmt::Display by default
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Structure(i32);
    /*impl fmt::Display for Structure {
        fn fmt(& self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "I'ma structure!")
        }
    }*/
    println!("This struct `{:#?}` won't print...", Structure(3));

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Person<'a> {
        name: &'a str,
        age: u8
    }
    println!("{:#?}", Person{name: "Hello", age: 27});

    // Print & format Pi value
    let pi = 3.141592;
    println!("Pi is roughly {pi:.3}");

    main_1_2_hw();
}

pub fn main_1_2_hw() {
    // Define a structure named `List` containing a `Vec`.
    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Extract the value using tuple indexing,
            // and create a reference to `vec`.
            let vec = &self.0;

            write!(f, "[")?;

            // Iterate over `v` in `vec` while enumerating the iteration
            // count in `count`.
            for (count, v) in vec.iter().enumerate() {
                // For every element except the first, add a comma.
                // Use the ? operator to return on errors.
                if count != 0 { write!(f, ", ")?; }
                write!(f, "{}: {}", count, v)?;
            }

            // Close the opened bracket and return a fmt::Result value.
            write!(f, "]")
        }
    }

    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}

