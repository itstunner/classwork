mod lib;

fn main() {
    let length = 15u32;
    let width = 10u32;
    // using relative path
    lib::projects::house::area(length, width); 
}
