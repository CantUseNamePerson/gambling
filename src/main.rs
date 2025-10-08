use rand::prelude::*;
use std::io;
fn main() {
    let mut money = String::new();
    println!("Enter starting money: ");
    io::stdin()
        .read_line(&mut money)
        .expect("Enter a starting number");
    let mut money: i32 = money.trim().parse().expect("Not a number");
    loop {
        println!("Always bet a value?(y/n)");
        let bet = yes();
        if bet == 'y' {
            println!("All in?(y/n)");
            if yes() == 'y' {
                loop {
                    if luck() {
                        money = money * 2;
                        println!("Lucky! Your money is now {}!", money)
                    } else {
                        money = 0;
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
                    let mut s = String::new();
                    io::stdin().read_line(&mut s).expect("Enter a number");
                    loop {
                        let bet: i32 = s.trim().parse().expect("Not a number");
                        if luck() {
                            money = money + bet;
                            println!("Lucky! Your money is now {}!", money);
                        } else {
                            money = money - bet;
                            println!("Unlucky! Your money is now {}!", money);
                        }
                        if money <= 0 {
                            break;
                        }
                    }
                }
            }
        }
        if money <= 0 {
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
