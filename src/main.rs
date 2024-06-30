use rand::Rng;
use std::io::{self, Write};

const SYMBOLS: [char; 6] = ['♠', '♥', '♦', '♣', '?', '7'];
const PAYOUTS: [u32; 6] = [10, 20, 30, 40, 0, 100]; // Corresponding payouts for '♠', '♥', '♦', '♣', '?', '7'

fn spin() -> [[char; 3]; 3] {
    let mut result = [[' '; 3]; 3];
    let mut rng = rand::thread_rng();
    for row in 0..3 {
        for col in 0..3 {
            result[row][col] = SYMBOLS[rng.gen_range(0..SYMBOLS.len())];
        }
    }
    result
}

fn check_line(line: [char; 3]) -> bool {
    if line[0] == '?' || line[1] == '?' || line[2] == '?' {
        let non_wildcard = line.iter().filter(|&&c| c != '?').collect::<Vec<&char>>();
        if non_wildcard.len() == 1 || non_wildcard.windows(2).all(|w| w[0] == w[1]) {
            return true;
        }
    } else {
        return line[0] == line[1] && line[1] == line[2];
    }
    false
}

fn calculate_payout(line: [char; 3]) -> u32 {
    let symbol_index = |c| SYMBOLS.iter().position(|&x| x == c).unwrap();
    
    if line[0] == '?' {
        return PAYOUTS[symbol_index(line[1])];
    } else if line[1] == '?' {
        return PAYOUTS[symbol_index(line[0])];
    } else if line[2] == '?' {
        return PAYOUTS[symbol_index(line[0])];
    } else {
        return PAYOUTS[symbol_index(line[0])];
    }
}

fn check_win(result: [[char; 3]; 3], lines: u32) -> u32 {
    let mut winnings = 0;
    if lines >= 1 && check_line(result[0]) {
        winnings += calculate_payout(result[0]);
    }
    if lines >= 2 && check_line(result[1]) {
        winnings += calculate_payout(result[1]);
    }
    if lines >= 3 && check_line(result[2]) {
        winnings += calculate_payout(result[2]);
    }
    if lines >= 4 {
        if check_line([result[0][0], result[1][1], result[2][2]]) {
            winnings += calculate_payout([result[0][0], result[1][1], result[2][2]]);
        }
        if check_line([result[0][2], result[1][1], result[2][0]]) {
            winnings += calculate_payout([result[0][2], result[1][1], result[2][0]]);
        }
    }
    winnings
}

fn display_slot_machine(balance: u32, lines: u32, bet_amount: u32, result: Option<[[char; 3]; 3]>, winnings: Option<u32>) {
    print!("\x1B[2J\x1B[1;1H"); // Clear the screen and move the cursor to the top left
    println!("Current balance: ${}", balance);
    println!("Lines: {}   Bet amount: ${}", lines, bet_amount);
    match winnings {
        Some(amount) if amount > 0 => println!("WIN ${}", amount),
        _ => println!(), // Print a blank line if no win
    }
    println!("+-----+-----+-----+");
    if let Some(res) = result {
        for row in &res {
            println!("|  {}  |  {}  |  {}  |", row[0], row[1], row[2]);
            println!("+-----+-----+-----+");
        }
    } else {
        for _ in 0..3 {
            println!("|     |     |     |");
            println!("+-----+-----+-----+");
        }
    }
    println!("(1) Change bet amount");
    println!("(2) Change lines");
    println!("(R) Repeat last bet");
    println!("(Q) Quit");
    println!("Press Enter to repeat last bet");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();
}

fn wait_for_enter() {
    let mut input = String::new();
    println!("Press Enter to continue...");
    io::stdin().read_line(&mut input).expect("Failed to read line");
}

fn main() {
    let mut balance = 100;
    let mut bet_amount = 1;
    let mut lines = 1;
    let mut play = String::new();
    let mut last_bet_amount = bet_amount;
    let mut last_lines = lines;

    loop {
        display_slot_machine(balance, lines, bet_amount, None, None);
        play.clear();
        io::stdin().read_line(&mut play).expect("Failed to read line");

        match play.trim().to_uppercase().as_str() {
            "1" => {
                print!("Enter new bet amount: ");
                io::stdout().flush().unwrap();
                play.clear();
                io::stdin().read_line(&mut play).expect("Failed to read line");
                bet_amount = match play.trim().parse() {
                    Ok(num) if num >= 1 && num <= balance => num,
                    _ => {
                        println!("Invalid input. Please enter a valid number.");
                        wait_for_enter();
                        continue;
                    }
                };
                last_bet_amount = bet_amount;
            }
            "2" => {
                print!("Enter number of lines to play (1-5): ");
                io::stdout().flush().unwrap();
                play.clear();
                io::stdin().read_line(&mut play).expect("Failed to read line");
                lines = match play.trim().parse() {
                    Ok(num) if num >= 1 && num <= 5 => num,
                    _ => {
                        println!("Invalid input. Please enter a number between 1 and 5.");
                        wait_for_enter();
                        continue;
                    }
                };
                last_lines = lines;
            }
            "R" | "" => {
                if balance < last_bet_amount * last_lines {
                    println!("You don't have enough balance to bet ${} on {} lines.", last_bet_amount, last_lines);
                    wait_for_enter();
                    continue;
                }

                balance -= last_bet_amount * last_lines;
                let result = spin();
                let winnings = check_win(result, last_lines);
                display_slot_machine(balance, last_lines, last_bet_amount, Some(result), Some(winnings));

                if winnings > 0 {
                    let total_winnings = winnings * last_bet_amount;
                    println!("Congratulations! You won ${}!", total_winnings);
                    balance += total_winnings;
                } else {
                    println!("Better luck next time!");
                }

                if balance == 0 {
                    println!("You are bankrupt. What would you like to do?");
                    println!("(D) Deposit more money");
                    println!("(Q) Quit");
                    play.clear();
                    io::stdin().read_line(&mut play).expect("Failed to read line");

                    match play.trim().to_uppercase().as_str() {
                        "D" => {
                            print!("Enter deposit amount: ");
                            io::stdout().flush().unwrap();
                            play.clear();
                            io::stdin().read_line(&mut play).expect("Failed to read line");
                            balance = match play.trim().parse() {
                                Ok(num) if num > 0 => num,
                                _ => {
                                    println!("Invalid input. Please enter a valid number.");
                                    wait_for_enter();
                                    continue;
                                }
                            };
                        }
                        "Q" => {
                            println!("You chose to quit. Thanks for playing!");
                            wait_for_enter();
                            return;
                        }
                        _ => {
                            println!("Invalid input. Please enter a valid option.");
                            wait_for_enter();
                        }
                    }
                } else {
                    wait_for_enter();
                }
            }
            "Q" => {
                println!("You cashed out with ${}. Thanks for playing!", balance);
                wait_for_enter();
                return;
            }
            _ => {
                println!("Invalid input. Please enter a valid option.");
                wait_for_enter();
            }
        }
    }
}