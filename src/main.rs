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
    In the future we'll show some distribution of percent succeeded at each round.
    Assumes that the student's function is "pure," so we can rerun it as many times
    as we'd like without promising that it'll get sequential turns.
*/

use dashu_ratio::RBig;
use itertools::Itertools;
use std::vec::Vec;

const TERMINATION_BOUND: RBig = frac_rbig(99, 100);
const WIN_SCORE: usize = 100;

#[allow(unused_variables)]
const fn student(score: usize) -> usize {
    match score {
        0 => 4,
        _ => 1,
    }
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

const fn frac_rbig(numerator: u128, denominator: u128) -> RBig {
    RBig::from_parts_const(dashu_base::Sign::Positive, numerator, denominator)
}

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

fn next_round(old_scores: &[RBig; WIN_SCORE + 1]) -> [RBig; WIN_SCORE + 1] {
    let mut new_scores = [RBig::ZERO; WIN_SCORE + 1];
    for (i, old_score) in old_scores.iter().enumerate().take(WIN_SCORE) {
        let to_roll: usize = STUDENT_ANSWERS[i];
        let num_worlds: RBig = frac_rbig(6, 1).pow(to_roll);

        let outcomes: Vec<usize> = all_worlds(i, to_roll);
        for outcome in outcomes {
            new_scores[outcome] += old_score / &num_worlds;
        }
    }
    new_scores[WIN_SCORE] += &old_scores[WIN_SCORE];
    new_scores
}

fn main() {
    // scores[n] has the fraction of "worlds" in which n is the current score
    let mut scores: [RBig; WIN_SCORE + 1] = [RBig::ZERO; WIN_SCORE + 1];
    scores[0] = frac_rbig(1, 1);
    // fraction of succeeded after round i+1
    // not used right now but I want it for the future
    // to show a distribution of when you won or something
    let mut successes: Vec<RBig> = vec![];

    while scores[WIN_SCORE] < TERMINATION_BOUND {
        scores = next_round(&scores);
        successes.push(scores[WIN_SCORE].clone());
    }

    println!(
        "Surpassed {} success in {} rounds!",
        TERMINATION_BOUND.to_f32_fast(),
        successes.len(),
    );
}
