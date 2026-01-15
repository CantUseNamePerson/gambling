use rand::prelude::*;
use std::io;
fn main() {
    // input money
    let mut money = String::new();
    println!("Enter starting money: ");
    io::stdin()
        .read_line(&mut money)
        .expect("Enter a starting number");
    // turn money var into int
    let mut money: i32 = money.trim().parse().expect("Not a number");
    // game loop
    loop {
        // ask if always bet a val or not
        println!("Always bet a value?(y/n)");
        let bet = yes();
        // if user says "yes"
        if bet == 'y' {
            // ask for all in or not
            println!("All in?(y/n)");
            // if user says "yes"
            if yes() == 'y' {
                // loop
                loop {
                    // if lucky
                    if luck() {
                        // money * 2
                        money = money * 2;
                        // print message
                        println!("Lucky! Your money is now {}!", money);
                    } else {
                        // money turns to 0
                        money = 0;
                        break;
                    }
                }
            } else {
                // ask for amount of percantage
                println!("Amount or percentage?(a/p)");
                // input
                let mut s = String::new();
                io::stdin().read_line(&mut s).expect("input 'a' or 'p'");
                // turn usr input into char
                let a = s.trim().chars().next().expect("No input detected");
                // if usr input if "Amount"
                if a == 'a' {
                    // input amount
                    println!("Enter amount: ");
                    let mut s = String::new();
                    io::stdin().read_line(&mut s).expect("Enter a number");
                    // loop
                    loop {
                        // turn usr input to int
                        let bet: i32 = s.trim().parse().expect("Not a number");
                        // if lucky
                        if luck() {
                            // add usr input to total money and print it
                            money = money + bet;
                            println!("Lucky! Your money is now {}!", money);
                        } else {
                            // subtract usr input to total money and print
                            money = money - bet;
                            println!("Unlucky! Your money is now {}!", money);
                        }
                        // if money if smaller than 0
                        if money <= 0 {
                            // break out of loop
                            break;
                        }
                    }
                }
            }
        }
        // if money smaller than 0
        if money <= 0 {
            // print and break
            println!("You lost all your money!");
            break;
        }
    }
}
// func that input thing, turn thing to char, then return it
fn yes() -> char {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("input 'y' or 'n'");
    let a = s.trim().chars().next().expect("No input detected");
    a
}
// see if lucky
fn luck() -> bool {
    let mut rng = rand::rng();
    let mut nums: Vec<i32> = (1..100).collect();
    nums.shuffle(&mut rng);
    nums.choose(&mut rng).unwrap() > &50
}
