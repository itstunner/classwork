pub mod welcome{ 
    pub fn table(data: u32){
        println!("we are in lib.rs -> table function");
        for count in 1..=4 {
        println!("{} * {} = {}",data, count, data*count);
        }
    }
}    