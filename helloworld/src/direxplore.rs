use std::env;

pub fn get_cur_dir() {
    let cur_dir = env::current_dir().unwrap();
    println!("Current directory: {}", cur_dir.display());
}

pub fn datatypes_explore() {
    let bool_flag: bool = true; //varname:datatype = value

    let small_int: i8 = 5; //signed integer 8 bit
    let small_uint: u8 = 5; //unsigned integer 8 bit
                            //default int in Rust is i32
    let small_float: f32 = 5.0; //32 bit float.  The . is mandatory
    let large_float: f64 = 5.0; //64 bit float

    println!("Min and max of i8: {} to {}", i8::MIN, i8::MAX);

    let some_char: char = 'a'; //single character, single quote

    let autoselectint: isize = 1000; //automatically selects int size based on system architecture

    let strslice = "Hello, World as str slice!"; //string slice : immutable ; compiled in code binary
    let str_original: String = String::from("Hello, World as String!"); //String : mutable ; heap allocated

    let str_conv: String = strslice.to_string(); //convert str slice to String
    let str_conv2: String = String::from("whatever"); //convert string literal to String
    let str_conv_rev: &str = &str_original; //convert String to str slice; dereference

    println!("Hello, World!");
}
