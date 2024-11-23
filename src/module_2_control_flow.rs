pub fn basic_if_else(){
    let transaction_amount:i32 = 0;
    if transaction_amount > 0 {
        println!("Transaction is valid.");
    } else if transaction_amount < 0 {
        println!("Invalid transaction: Negative amount.");
    } else {
        println!("Transaction amount is 0. No tranfer!");
    }
}

pub fn match_example(day: u8){
 let block_day = match day {
        1 => "Block production on Monday.",
        2 => "Validation rewards on Tuesday.",
        3 => "Transaction settlement on Wednesday.",
        4 => "Governance voting on Thursday.",
        5 => "Network upgrade on Friday.",
        6 => "Node maintenance on Saturday.",
        7 => "No activity on Sunday.",
        _ => "Invalid block day."
    };
    println!("Blockchain activity: {}", block_day);
}

pub fn while_loop_example(){
    let mut pending_transaction: i32 = 0;
    while pending_transaction < 5 {
        println!("Processing transaction number : {}",
                    pending_transaction + 1
                );
        pending_transaction += 1;
    }

    println!("All transaction processed.");
}

pub fn for_loop_example(){
    let staking_rewards = [10, 20, 30, 40, 50];
    for reward in staking_rewards.iter(){
        println!("Validator reward: {}", reward);
    }
    for block in 1..5{
        println!("Produced block number: {}", block);
    }
}

pub fn infinite_loop(){
    let mut attemps:i32 = 0;
    loop {
        println!("Checking blockchain state...attemps : {}", attemps+1);
        attemps+=1;
        if attemps==3{
            println!("Breaking attemps after 3 attemps");
            break;
        }
    }
}

pub fn match_pattern_example(number: i32){
    match number{
        1 => println!("Executing token transfer."),
        2 | 3 | 4 | 7 => println!("Executing a prime validator operation."),
        10..=19 => println!("Performing governance between block 10 and 19."),
        _ => println!("Unrecognized operation."),
    }

}

pub fn match_return_example(status_code: i32) -> &'static str {
    match status_code {
        200 => "Transaction successful.",
        201 => "Transaction created successfully.",
        202 => "Transaction accepted for processing.",
        400 => "Invalid transaction request.",
        401 => "Unauthorized access to the blockchain.",
        403 => "Transaction forbidden.",
        404 => "Transaction not found.",
        409 => "Transaction conflict detected.",
        500 => "Internal blockchain error.",
        503 => "Blockchain service unavailable.",
        _ => "Unknown status.",
    }
}


pub fn demo(){
    println!("\n");
    // basic_if_else();
    // match_example(1);
    //while_loop_example();
    //for_loop_example();
    //infinite_loop();
   // match_pattern_example(2);
   let status_message: &str = match_return_example(500);
   println!("Status message: {}", status_message);
}