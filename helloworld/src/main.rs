use direxplore::get_cur_dir;

pub mod direxplore;

#[allow(unused_assignments)] //prevent warning for unused assignment
#[allow(unused_variables)] //prevent warning for unused variable

// convention : use snake case for variable names
// Rust assumes variables are immutable by default
// To make a variable mutable, use the mut keyword

fn main() {
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

    get_cur_dir();
}
