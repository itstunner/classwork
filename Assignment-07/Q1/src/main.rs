use std :: io;

mod users{
    pub mod experts{
        pub fn operation(input1: u32, input2: u32){
                let sum = input1 + input2;
                println!("The sum is : {}", sum);
        }
    } 
}

fn main() {
    println!("we are doing Arithmatic and logic operation");
    
    // input two different numbers from user 
    println!("Enter the first number");
    let mut number1 = String :: new();
    io :: stdin().read_line(&mut number1).expect("invalid data");
    let number1 : u32 = number1.trim().parse().expect("Try again");

    println!("Enter the second number");
    let mut number2 = String :: new();
    io :: stdin().read_line(&mut number2).expect("invalid data");
    let number2 : u32 = number2.trim().parse().expect("Try again");

    // now we are using relative path
    users :: experts :: operation(number1,number2);
}
