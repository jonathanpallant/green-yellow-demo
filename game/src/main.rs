//! The Green and Yellow Game
//!
//! It's not like that game that rhymes with Curdle.

use libgreenyellow::calc_green_and_yellow;

fn main() {
    // 0. Say hello
    println!("Welcome to Green and Yellow. Guess my secret digits!");

    // 1. Pick a secret number
    let secret = pick_secret();
    // println!("Ssssshhhh....secret is {:?}", secret);
    // 2. Go into a loop
    loop {

        // 2a. Get a guess from the user (as a string)
        // 2b. Convert the guess into digits ([u8; 4])
        let guess = match get_guess() {
            Ok(g) => g,
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        };

        // 2c. Check if guess is any good
        let output = calc_green_and_yellow(&guess, &secret);

        // 2d. Print the results
        println!("{output}");

        // 2e. If guess is correct, exit!
        if guess == secret {
            println!("Congratulations!");
            break;
        }
    }
}

/// Pick four random digits
fn pick_secret() -> [u8; 4] {
    use rand::Rng;

    let mut rng = rand::rng();
    let mut output = [0u8; 4];
    for slot in output.iter_mut() {
        *slot = rng.random_range(1..=9);
    }

    output
}

/// Take four random digits from standard input
fn get_guess() -> Result<[u8; 4], anyhow::Error> {
    println!("Enter guess:");
    let stdin = std::io::stdin();
    let mut output = [0u8; 4];
    let mut buf = String::new();
    stdin.read_line(&mut buf)?;

    for (digit, slot) in buf.split_whitespace().zip(output.iter_mut()) {
        let Ok(x) = digit.parse() else {
            anyhow::bail!("{} is not a valid digit", digit);
        };
        *slot = x;
    }

    Ok(output)
}
