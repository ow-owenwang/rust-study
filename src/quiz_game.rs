#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! rand = "0.8"
//! ```

use rand::Rng;
use std::cmp::Ordering;
use std::io;

/*
 * 程序生成一个介于1和100之间的随机证书，然后提示玩家输入猜测。输入猜测后，程序将指示猜测是过低还是过高。如果猜测正确，游戏将打印一条祝贺信息并退出
 */

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
