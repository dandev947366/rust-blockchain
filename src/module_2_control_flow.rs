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

pub fn demo(){
    println!("\n");
    // basic_if_else();
    match_example(1);
}