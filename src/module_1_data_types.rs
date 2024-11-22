pub fn primitive_data_types(){
    let token_supply: u128 = 1_000_000_000_000_000_000;
    let block_number: i64 = -1234567890;
    let token_price: f32 = 3.14;
    let transaction_fee:f64 = 0.000001;
    let is_transaction_valid:bool = true;
    let token_symbol:char = 'T';
    let wallet_address:&str="0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045";
    let contract_address:String = String::from("Decentralized Exchange");
    let gas_fee:f64 = 0.00000002;
    let gas_used:f64 = 2100.0;
    println!("Wallet address (&str) : {}", wallet_address);
    println!("Contract address (String) : {}", contract_address);
    println!("\n");
    println!("Token symbol (char) : {}", token_symbol);
    println!("Token supply (u128) : {}", token_supply);
    println!("Block number (i64) : {}", block_number);
    println!("Token Price (f32) : {}", token_price);
    println!("Transaction fee (f64) : {}", transaction_fee);
    println!("Transaction validation (bool) : {}", is_transaction_valid);
    println!("Gas fee: {}, Gas used: {}", gas_fee, gas_used);
    println!("Total gas fee: {:.8}", gas_fee*gas_used);

}

pub fn arithmetic_operations(){
    let account_balance:i32 = 1000;
    let transaction_amount:i32 = 250;

    println!(
        "Account Balance: {}, Transaction Amount: {}",
        account_balance, transaction_amount
    );

    println!(
        "New balance after transaction: {}",
        account_balance - transaction_amount
    );

    println!(
        "Doubling transaction amount (for staking) : {}",
        transaction_amount*2
    );

    println!(
        "Division for share distribution: 1000/4 : {}",
        account_balance/4
    );

    println!(
        "Remainder when dividing transaction fee: 1000%3 : {}",
        account_balance%3
    );
}

pub fn logication_operations(){
    let is_staking:bool = true;
    let has_sufficient_balance:bool = false;

    println!("Is staking: {}, Has sufficient balance: {}", is_staking, has_sufficient_balance);
    println!("Can perform staking: {}", is_staking&&has_sufficient_balance);
    println!("Can either perform staking or withdraw: {}", is_staking||has_sufficient_balance);
    println!("Negating staking status (!staking): {}", !is_staking);

}

pub fn variable_shadowing_n_conversion(){
    let account_balance: i32 = 500;
    println!("Account balance: {}", account_balance);
    let account_balance:i32 = account_balance+100;
    println!("Updated account balance: {}", account_balance);
    let gas_fee:f64 = 4.0025;
    let gas_fee_int:i32 = gas_fee as i32;
    println!("Gas fee: {}, Converted to lamport: {}", gas_fee, gas_fee_int);
    let block_height:i32 = 128550;
    let block_height_str:String = block_height.to_string();
    println!("Block height: {}, Block height string: {}", block_height, block_height_str);
}

pub fn mutability_example(){
    let _token_supply:i32 = 2500;
   // token_supply=2000; Erro =>> cannot assign twice to immutable variable
    let mut account_balance:i32 = 2500;
    println!("Before transaction: User balance = {}", account_balance);
    account_balance -= 50;
    println!("After transaction: User balance = {}", account_balance);
}

pub fn tuple_destructuring_example(){
    let transaction_info: (&str, i32, f64) = ("Transfer", 200, 0.002);
    let (tx_type, tx_amount, tx_fee) = transaction_info;
    println!("Transaction Type: {}, Amount {}, Fee: {}", tx_type, tx_amount, tx_fee);
    println!("Transaction Type: {}, Amount {}, Fee: {}", transaction_info.0, transaction_info.1, transaction_info.2);

}
pub fn demo(){
    primitive_data_types();
    println!("\n");
    arithmetic_operations();
    println!("\n");
    logication_operations();
    println!("\n");
    variable_shadowing_n_conversion();
    println!("\n");
    mutability_example();
    println!("\n");
    tuple_destructuring_example();
}

