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

use itertools::Itertools;
use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;
use std::vec::Vec;

const TERMINATION_BOUND: f64 = 0.95;
const WIN_SCORE: usize = 100;

#[allow(unused_variables)]
const fn student(score: usize) -> usize {
    3 // change to see different strategies!
}

const STUDENT_ANSWERS: [usize; WIN_SCORE] = {
    let mut rolled = [0; WIN_SCORE];
    let mut i = 0;
    let mut stud_ret;
    while i < WIN_SCORE {
        stud_ret = student(i);
        assert!(!(stud_ret == 0), "Student can never return 0 dice to roll!");
        rolled[i] = stud_ret;
        i += 1;
    }
    rolled
};

fn all_worlds(curr_score: usize, to_roll: usize) -> Vec<usize> {
    use itertools::FoldWhile;
    use itertools::FoldWhile::{Continue, Done};

    std::iter::repeat_n(1..=6, to_roll)
        .multi_cartesian_product()
        .map(|set_of_rolls| {
            let added_score: FoldWhile<Option<usize>> =
                set_of_rolls
                    .iter()
                    .fold_while(Some(0), |acc, new| match (new, acc) {
                        (1, Some(_)) => Continue(None),
                        (1, None) => Done(None),
                        (x, Some(y)) => Continue(Some(x + y)),
                        (_, None) => Continue(None),
                    });
            match added_score {
                Continue(Some(x)) => (x + curr_score).min(WIN_SCORE),
                Continue(None) => curr_score,
                Done(_) => 0,
            }
        })
        .collect()
}

fn next_round(old_scores: &[BigUint; WIN_SCORE + 1]) -> [BigUint; WIN_SCORE + 1] {
    let mut new_scores = [BigUint::ZERO; WIN_SCORE + 1];
    for (i, old_score) in old_scores.iter().enumerate().take(WIN_SCORE) {
        let to_roll: usize = STUDENT_ANSWERS[i];
        let outcomes: Vec<usize> = all_worlds(i, to_roll);
        for outcome in outcomes {
            new_scores[outcome] += old_score;
        }
    }
    new_scores
}

fn win_percent(percents: &Vec<(BigUint, BigUint)>) -> f64 {
    let mut base: f64 = 0.0;
    for (numer, denom) in percents {
        base += (1.0 - base) * (numer.to_f64().unwrap()) / (denom.to_f64().unwrap());
    }
    base
}

fn main() {
    // scores[n] has the number of "worlds" in which n is the current score
    let mut scores: [BigUint; WIN_SCORE + 1] = [BigUint::ZERO; WIN_SCORE + 1];
    scores[0] = BigUint::from(1_u8);

    // store as rationals because float imprecision is getting us
    let mut successes: Vec<(BigUint, BigUint)> = vec![];
    let mut how_many_have_won = win_percent(&successes);

    while how_many_have_won < TERMINATION_BOUND {
        scores = next_round(&scores);

        successes.push((scores[WIN_SCORE].clone(), scores.iter().sum::<BigUint>()));

        scores[WIN_SCORE] = BigUint::ZERO;

        how_many_have_won = win_percent(&successes);
    }

    println!(
        "Achieved {:.3} success in {} rounds!",
        how_many_have_won,
        successes.len()
    );
}
