pub fn table1(data: u32){
    println!("we are in library package -> table1 function");
    for count in 1..=3 {
        println!("{} * {} = {}",data, count, data*count);
    }
}