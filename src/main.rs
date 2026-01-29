use rand::prelude::*;
use colored::*;
use std::io;
use std::io::Write;

fn main() {
    println!("{}", "Ласкаво просимо в гру Вгадай число!".cyan());
    let computer_number = thread_rng().gen_range(1..101);
    let mut attempts = 0;
    loop {
        print!("{}", "Введіть своє число: ".cyan());
        io::stdout().flush().unwrap();
        let mut user_number = String::new();
        io::stdin().read_line(&mut user_number).unwrap();
        let user_number: u8 = match user_number.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        attempts += 1;

        if user_number < computer_number && attempts < 10 {
            println!("{}", "🤏 Замало".red());
        } else if user_number > computer_number && attempts < 10 {
            println!("{}", "🤏 Забагато".red());
        } else if user_number == computer_number {
            println!("{}", "✌️ Ви перемогли!".green());
            print!("{}", "К-ть попиток: ".green());
            println!("{}", attempts);
            break;
        } else if attempts >= 10 {
            println!("{}", "😔 Ви програли".red());
            break;
        }
    }
}