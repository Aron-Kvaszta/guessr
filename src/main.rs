//1  TODO: Generate random number every time the app runs
//2  TODO: Get user input
//3  TODO: Parse user input to a number
//4  TODO: Compare user input and generated number ->
//5  TODO:    if user input > generated number, tell user their number is too high
//6  TODO:    if user input < generated number, tell user their number is too low
//7  TODO:    if user input == generated number, tell user that their number is the one the computer generated
//8  TODO: Ask user if they want to play again
//9  TODO: If user types in "Y" -> restart from step 1
//10 TODO: If user types in "N" -> exit the program

use rand::Rng;
use std::io;

fn main() {
    loop {
        if game_loop() == false {
            break;
        }
    }
}

fn game_loop() -> bool {
    let random_number:i32 = rand::thread_rng().gen_range(1..100);

    // println!("Computer generated number is: {}", random_number);

    println!("The computer has generated a number!\nTry guessing it!\n");
    println!("Type in your guess:");

    loop {
        let mut user_number: String = String::new();
        io::stdin()
            .read_line(&mut user_number)
            .expect("Failed to read input.");

        // println!("Your number is: {}", user_number);

        if match_numbers(user_number, random_number) {
            break;
        }
    }

    let mut replay_prompt:String = String::new();
    println!("Do you want to play again? (Y/Anything Else)");

    io::stdin()
        .read_line(&mut replay_prompt)
        .expect("Failed to parse input");

    if replay_prompt.trim().to_uppercase() == "Y" {
        return true;
    }
    else {
        return false;
    }
}

fn match_numbers(num:String, rand_num:i32) -> bool {
    let num:i32 = num.trim().parse().expect("Failed to parse");

    if num < rand_num {
        println!("Your number ({}) is less than the generated one", num);
        return false;
    }
    else if num > rand_num {
        println!("Your number ({}) is greater than the generated one", num);
        return false;
    }
    else {
        println!("That's it! The generated number was {}", rand_num);
        return  true;
    }
}