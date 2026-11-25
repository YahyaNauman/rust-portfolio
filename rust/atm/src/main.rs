use std::io;

fn main() {
    let mut balance: u64 = 0;

    println!(
        "enter action \n\
         balance = 1 \n\
         deposite = 2 \n\
         withdraw = 3 \n\
         exit = 4 "
    );

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let mut action: u64 = input.trim().parse().expect("Please type a valid number!");

    while action != 4 {
        if action == 1 {
            println!("balance : {balance}");
        } else if action == 2 {
            println!("enter deposit amount:");
            let mut amt_input = String::new();
            io::stdin()
                .read_line(&mut amt_input)
                .expect("Failed to read line");
            let amount: u64 = amt_input
                .trim()
                .parse()
                .expect("Please type a valid number!");

            balance += amount;
            println!("balance : {balance}");
        } else if action == 3 {
            println!("enter withdraw amount:");
            let mut amt_input = String::new();
            io::stdin()
                .read_line(&mut amt_input)
                .expect("Failed to read line");
            let amount: u64 = amt_input
                .trim()
                .parse()
                .expect("Please type a valid number!");

            if amount > balance {
                println!("insufficient balance! current balance : {balance}");
            } else {
                balance -= amount;
                println!("balance : {balance}");
            }
        } else {
            println!("unknown action: {action}");
        }

        println!(
            "enter action \n\
             balance = 1 \n\
             deposite = 2 \n\
             withdraw = 3 \n\
             exit = 4 "
        );

        let mut next_input = String::new();
        io::stdin()
            .read_line(&mut next_input)
            .expect("Failed to read line");
        action = next_input
            .trim()
            .parse()
            .expect("Please type a valid number!");
    }

    println!("exiting...");
}
