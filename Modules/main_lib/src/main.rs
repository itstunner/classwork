use std::io;
use table;
mod lib;

mod printing{
    pub mod math {
        #[derive(Debug)]
        pub struct Data {
            pub name: String,
            pub score: u8, 
        }

        #[derive(Debug)]
        pub enum Direction {
            Left,
            Right,
            Up,
            Down,
        }

        pub fn table(data: u32){
            println!("we are in table function");
            for count in 1..=3 {
                println!("{} * {} = {}",data, count, data*count);
            }
        }
    }
}

fn main() {
    loop {
    println!("Please input your number");

    let mut input1 = String::new();

    io::stdin().read_line(&mut input1).expect("Failed to read line");

    //let input1: u32 = input1.trim().parse().unwrap();
    let input1: u32 = match input1.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    println!("You entered: {}", input1);

    crate::printing::math::table(input1); // absolute path
    printing::math::table(input1); // relative path
    lib::welcome::table(input1);
    table::table1(input1);
    let student = printing :: math :: Data {
        name : "Fawad Ahmad".to_string(),
        score : 23,
    }; 

    let sign = printing::math::Direction::Left;

    println!("{:?}", student);
    println!("{:?}", sign);

    break;
    }
}

