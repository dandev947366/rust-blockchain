fn primitive_data_types(){
    let token_supply: u128 = 1_000_000_000_000_000_000;
    let block_number: i64 = -1234567890;
    let token_price: f32 = 3.14;
    let transaction_fee:f64 = 0.000001;
    println!("Token supply (u128) : {}", token_supply);
    println!("Block number (i64) : {}", block_number);
    println!("Token Price (u128) : {}", token_price);
    println!("Transaction fee (i64) : {}", transaction_fee);
}


pub fn demo(){
    primitive_data_types();
}

