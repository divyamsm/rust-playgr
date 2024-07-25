use direxplore::{datatypes_explore, get_cur_dir};

pub mod direxplore;

#[allow(unused_assignments)] //prevent warning for unused assignment
#[allow(unused_variables)] //prevent warning for unused variable

// convention : use snake case for variable names
// Rust assumes variables are immutable by default
// To make a variable mutable, use the mut keyword

fn main() {
    datatypes_explore();
    get_cur_dir();
}
