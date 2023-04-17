use rand::prelude::SliceRandom;
use std::{io::Write};
use std::num::ParseIntError;

/**
 * Generate Random Number Between 0 and 9
 */
fn get_random_number() -> [u8; 4] {
    let mut rng = rand::thread_rng();
    let mut numbers = vec![0,1,2,3,4,5,6,7,8,9];
    numbers.shuffle(&mut rng);
    [numbers[0], numbers[1], numbers[2], numbers[3]]
}

/**
 * Parse input
 */
fn parse_input(input: &String) -> Result<Vec<u8>, String> {
    //check input has ' ' char
    let check = input.trim().find(' ');

    let nums = match check
    {
        None => input_as_numeric(input)?,
        _ => input_as_string(input)?
    };

    Ok(nums)
}

fn input_as_string(input: &String) -> Result<Vec<u8>, String> {
    let nums: Vec<Result<u8, String>> = input.split_whitespace()
    .map(|s| s.parse().map_err(|error: ParseIntError| error.to_string()))
    .collect();
    
    let mut e_msg = None;
    
    let parsed_nums: Vec<u8> = nums.into_iter()
    .map(|num| {
        match num {
            Ok(n) => n,
            Err(e) => {
                e_msg.insert(e);
                0
            }
        }
    })
    .collect();
    
    if e_msg != None{
        return Err(e_msg.unwrap());
    }

    Ok(parsed_nums)
}

fn input_as_numeric(input: &String) -> Result<Vec<u8>, String>{
    let mut nums: Vec<u8> = vec!();

    for i in input.trim().chars()
    {
        if !i.is_ascii_digit() { // char가 숫자가 아닌 경우
            return Err("Invalid Input".to_string());
        }
        let c = i as u8;
        nums.push(c - 48);
    }
    
    Ok(nums)
}

struct Game {
    target_number: [u8; 4],
    try_count: u32
}

impl Game {
    fn guess_number(&mut self, guess_number: Vec<u8>) -> (u8, u8) {
        // Check Strike or Ball
        if guess_number.len() != 4 {
            return (255, 255);
        }
        let mut strike: u8 = 0;
        let mut ball: u8 = 0;
        for i in 0..4 {
            // check strike
            if self.target_number[i] == guess_number[i] {
                strike+=1;
                continue;
            }
            for j in 0..4 {
                // check ball
                if self.target_number[i] == guess_number[j] {
                    ball+=1;
                    break;
                }
            }
        }
        self.try_count += 1;
        (strike, ball)
    }
}
fn main() {
    // Game main loop
    'main_loop: loop {
        println!("====================");
        println!("   Bulls and Cows   ");
        println!("====================");
        println!("");
        let mut game = Game {
            target_number : get_random_number(),
            try_count : 0
        };
        println!("{:?}", game.target_number);
        loop {
            
            //input loop
            let mut number :Result<Vec<u8>, _>;
            loop {
                let mut input = String::new();
                print!("Input your guess: ");
                std::io::stdout().flush().expect("Flush Failed.");
                std::io::stdin().read_line(&mut input).expect("STDIN read_line Failed.");
                number = parse_input(&input);

                if number.is_ok() {
                    break;
                }
                println!("{:?}", number);
            }

            let guessed_number = number.unwrap();
            
            if guessed_number[0] > 10 {
                break 'main_loop;
            }
            let (strike, ball) = game.guess_number(guessed_number);
            println!("Strike: {strike} Ball: {ball}");
            if strike == 4 {
                println!("Win!");
                break;
            }
        }
    }
    println!("Bye.");
}
