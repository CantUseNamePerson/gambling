use rand::prelude::*;
use rust_decimal::prelude::*;
use std::io;
fn main() {
    let mut money = String::new();
    println!("Enter starting money:");
    io::stdin().read_line(&mut money).expect("Failed to read line");
    let mut money = Decimal::from_str(money.trim()).expect("Not a valid decimal number");
    loop {
        println!("Always bet a value?(y/n)");
        let bet = yes();
        if bet == 'y' {
            println!("All in?(y/n)");
            if yes() == 'y' {
                loop {
                    if luck() {
                        money = money * Decimal::from(2);
                        println!("Lucky! Your money is now {}!", money);
                    } else {
                        money = Decimal::from(0);
                        break;
                    }
                }
            } else {
                println!("Amout or percentage?(a/p)");
                let mut s = String::new();
                io::stdin().read_line(&mut s).expect("input 'a' or 'p'");
                let a = s.trim().chars().next().expect("No input detected");
                if a == 'a' {
                    println!("Enter amount: ");
                    let mut bet = String::new();
                    println!("Enter amount to bet: ");
                    io::stdin()
                        .read_line(&mut bet)
                        .expect("Enter a decimal number");
                    let mut bet = Decimal::from_str(bet.trim()).expect("Enter decimal number");
                    loop {
                        if money <= Decimal::from(0) {
                            break;
                        } if luck() {
                            money = money + bet;
                            println!("Lucky! Your money is now {}!", money);
                        } else {
                            money = money - bet;
                            println!("Unlucky! Your money is now {}!", money);
                        }
                    }
                } else {
                    println!("Enter percentage: ");
                    let mut bet = String::new();
                    io::stdin()
                        .read_line(&mut bet)
                        .expect("Enter a decimal number");
                    let mut bet = Decimal::from_str(bet.trim()).expect("Enter decimal number");
                    loop {
                        bet = money * (bet / Decimal::from(100));
                        if luck() {
                            money = money + bet;
                            money = money.round_dp(2);
                            println!("Lucky! Your money is now {}!", money);
                        } else {
                            money = money - bet;
                            money = money.round_dp(2);
                            println!("Unlucky! Your money is now {}!", money);
                        }
                        if money <= Decimal::from(0) {
                            break;
                        }
                    }
                }
            }
        }
        if money <= Decimal::from(0) {
            println!("You lost all your money!");
            break;
        }
    }
}
fn yes() -> char {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("input 'y' or 'n'");
    let a = s.trim().chars().next().expect("No input detected");
    a
}
fn luck() -> bool {
    let mut rng = rand::rng();
    let mut nums: Vec<i32> = (1..100).collect();
    nums.shuffle(&mut rng);
    nums.choose(&mut rng).unwrap() > &50
}
