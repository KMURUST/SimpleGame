use rand::prelude::SliceRandom;
use std::io::Write;

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
fn parse_input(input: &String) -> Vec<u8> {
    let nums: Vec<u8> = input.split_whitespace()
    .map(|s| s.parse().expect("Invalid Input"))
    .collect();
    nums
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
            let mut input = String::new();
            print!("Input your guess: ");
            std::io::stdout().flush().expect("Flush Failed.");
            std::io::stdin().read_line(&mut input).expect("STDIN read_line Failed.");
            let guessed_number = parse_input(&input);
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
