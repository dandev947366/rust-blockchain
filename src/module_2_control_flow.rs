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


pub fn demo(){
    println!("\n");
    // basic_if_else();
    // match_example(1);
    //while_loop_example();
    //for_loop_example();
    infinite_loop();
}