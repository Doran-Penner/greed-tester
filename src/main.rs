/*
    Every round in "greed," the player chooses a number n and rolls nd6.
    If there's at least one 1, the player gets no points this round;
    if there are two or more, the player's score resets to 0.
    Otherwise, the sum of the dice is added to the player's total score,
    which starts at 0. Get to 100 in as few rounds as possible!

    This code is meant to test how effective a given strategy is. We're simplifying
    it a bit --- now it's a single-player game and the strategy function only takes as
    input its current score, which will be between 0 and 99 (100+ being game over). For
    a pre-defined function `student` which returns the number of dice to roll given
    their current score, this code will simulate "all possible outcomes" to find how good
    the strategy is in the long run. Since this will never terminate without a bound,
    `TERMINATION_BOUND` declares the percentage of completeness at which the simulation
    should stop. For example, if it equals 0.5, then this program will stop when 50% of
    the scenarios have completed the game and report how many rounds it took.
    Assumes that the student's function is "pure," so we can rerun it as many times
    as we'd like without promising that it'll get sequential turns.
*/

use std::vec::Vec;
use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;


const TERMINATION_BOUND: f64 = 0.95;
const WIN_SCORE: usize = 100;


#[allow(unused_variables)]
fn student(score: usize) -> usize {
    3  // change to see different strategies!
}


fn sum(lst: &[BigUint]) -> BigUint {
    let mut ret: BigUint = BigUint::ZERO;
    for x in lst {
        ret += x;
    }
    ret
}


fn _gen_rolls(to_roll: usize) -> Vec<Vec<usize>> {
    if to_roll == 1 {
        vec![vec![1], vec![2], vec![3], vec![4], vec![5], vec![6]]
    } else {
        let mut ret: Vec<Vec<usize>> = Vec::new();
        for i in 1..=6 {
            let mut smaller: Vec<Vec<usize>> = _gen_rolls(to_roll - 1);
            for list in &mut smaller {
                list.push(i)
            }
            ret.extend(smaller);
        };
        ret
    }
}


fn gen_rolls(to_roll: usize) -> Vec<Vec<usize>> {
    assert!(to_roll > 0, "Must provide a positive number of dice to roll!");
    // NOTE could memoize these calls for more performance if needed
    _gen_rolls(to_roll)
}


fn all_worlds(curr_score: usize, to_roll: usize) -> Vec<usize> {
    let all_rolls: Vec<Vec<usize>> = gen_rolls(to_roll);
    let mut ret: Vec<usize> = Vec::new();

    for set_of_rolls in all_rolls {
        let mut ones_found: usize = 0;
        let mut sum_rolls: usize = 0;
        for roll in set_of_rolls {
            if roll == 1 {
                ones_found += 1;
            } else {
                sum_rolls += roll;
            }
        }

        let final_score: usize = match ones_found {
            0 => if sum_rolls + curr_score > WIN_SCORE { WIN_SCORE }
                 else { sum_rolls + curr_score },
            1 => curr_score,
            _ => 0
        };
        ret.push(final_score)
    }
    ret
}


fn next_round(scores: [BigUint; WIN_SCORE + 1]) -> [BigUint; WIN_SCORE + 1] {
    let mut new_scores: [BigUint; WIN_SCORE + 1] = [BigUint::ZERO; WIN_SCORE + 1];
    for i in 0..WIN_SCORE {
        // NOTE could memoize `student` as well
        let to_roll: usize = student(i);
        let outcomes: Vec<usize> = all_worlds(i, to_roll);
        for outcome in outcomes {
            new_scores[outcome] += scores[i].clone();  // TODO do I need clone?
        }
    }
    new_scores
}


fn main() {
    // scores[n] has the number of "worlds" in which n is the current score
    let mut scores: [BigUint; WIN_SCORE + 1] = [BigUint::ZERO; WIN_SCORE + 1];
    scores[0] = BigUint::from(1 as u8);
    let mut round: usize = 0;

    let mut succeeded: f64 = 0.0;
    while succeeded < TERMINATION_BOUND {
        scores = next_round(scores);

        succeeded += (1.0 - succeeded)
                   * scores[WIN_SCORE].to_f64().expect("biguint->flaot conversion failure")
                   / sum(&scores).to_f64().expect("biguint->flaot conversion failure");
        scores[WIN_SCORE] = BigUint::ZERO;

        round += 1;
    }

    println!("Achieved {succeeded}% success in {round} rounds!")
}
