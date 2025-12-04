//! # libgreenyellow
//!
//! Contains useful functionality for implementing the game Green and Yellow.

pub struct Game {
    secret: [u8; 4]
}

/// Calculate the score for a single guess in the game Green and Yellow.
///
/// Given four secret digits, and four attempts at guessing those digits you will get a four character output.
///
/// * Green block means right digit in the right place
/// * Yellow means right digit but in the wrong place
/// * Grey means guess digit not in secret
///
/// ```rust
/// use libgreenyellow::calc_green_and_yellow;
/// let s = calc_green_and_yellow(&[1, 2, 3, 4], &[2, 2, 3, 4]);
/// assert_eq!(s, "⬜🟩🟩🟩");
/// ```
pub fn calc_green_and_yellow(guess: &[u8; 4], secret: &[u8; 4]) -> String {
    let mut output: [char; 4] = ['⬜'; 4];
    let mut used: [bool; 4] = [false; 4];

    for idx in 0..4 {
        if guess[idx] == secret[idx] {
            output[idx] = '🟩';
            used[idx] = true;
        }
    }

    for guess_idx in 0..4 {
        for secret_idx in 0..4 {
            if guess_idx == secret_idx {
                continue;
            }
            if output[guess_idx] == '🟩' {
                continue;
            }
            if guess[guess_idx] == secret[secret_idx] && !used[secret_idx] {
                used[secret_idx] = true;
                output[guess_idx] = '🟨';
            }
        }
    }

    output.iter().collect()
}

#[cfg(test)]
mod tests;
