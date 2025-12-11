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

use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;
use std::vec::Vec;

const TERMINATION_BOUND: f64 = 0.95;
const WIN_SCORE: usize = 100;

#[allow(unused_variables)]
const fn student(score: usize) -> usize {
    3 // change to see different strategies!
}

const STUDENT_ANSWERS: [usize; WIN_SCORE] = student_list::<WIN_SCORE>();

const fn student_list<const N: usize>() -> [usize; N] {
    let mut rolled = [0; N];
    let mut i = 0;
    let mut stud_ret;
    while i < N {
        stud_ret = student(i);
        if stud_ret == 0 {
            panic!("Student can never return 0 dice to roll!")
        }
        rolled[i] = stud_ret;
        i += 1;
    }
    rolled
}

fn gen_rolls(to_roll: usize) -> Vec<Vec<usize>> {
    assert!(
        to_roll > 0,
        "Must provide a positive number of dice to roll!"
    );
    if to_roll == 1 {
        vec![vec![1], vec![2], vec![3], vec![4], vec![5], vec![6]]
    } else {
        (1..=6)
            .flat_map(|i| {
                let mut smaller: Vec<Vec<usize>> = gen_rolls(to_roll - 1);
                smaller.iter_mut().for_each(|list| list.push(i));
                smaller
            })
            .collect()
    }
}

fn all_worlds(curr_score: usize, to_roll: usize) -> Vec<usize> {
    gen_rolls(to_roll)
        .iter()
        .map(|set_of_rolls| {
            let added_score: Option<Option<usize>> = set_of_rolls
                .iter()
                // we abuse try_fold to get short-circuiting fold;
                // could use itertools.fold_while but whatever
                .try_fold(
                    Some(0),
                    |acc: Option<usize>, new| -> Option<Option<usize>> {
                        match (new, acc) {
                            (1, Some(_)) => Some(None),
                            (1, None) => None,
                            (x, Some(y)) => Some(Some(x + y)),
                            (_, None) => Some(None),
                        }
                    },
                );
            match added_score {
                Some(Some(x)) => (x + curr_score).min(WIN_SCORE),
                Some(None) => curr_score,
                None => 0,
            }
        })
        .collect()
}

fn next_round(old_scores: [BigUint; WIN_SCORE + 1]) -> [BigUint; WIN_SCORE + 1] {
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

fn main() {
    // scores[n] has the number of "worlds" in which n is the current score
    let mut scores: [BigUint; WIN_SCORE + 1] = [BigUint::ZERO; WIN_SCORE + 1];
    scores[0] = BigUint::from(1u8);
    let mut round: usize = 0;

    let mut succeeded: f64 = 0.0;
    while succeeded < TERMINATION_BOUND {
        scores = next_round(scores);

        succeeded += (1.0 - succeeded)
            * scores[WIN_SCORE]
                .to_f64()
                .expect("biguint->float conversion failure")
            / scores
                .iter()
                .sum::<BigUint>()
                .to_f64()
                .expect("biguint->float conversion failure");
        scores[WIN_SCORE] = BigUint::ZERO;

        round += 1;
    }

    println!("Achieved {succeeded:.3} success in {round} rounds!")
}
