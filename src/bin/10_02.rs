use std::i32;

use good_lp::{Expression, IntoAffineExpression, Solution, SolverModel, constraint, variable};

fn sum(input: &str) -> i32 {
    input
        .trim()
        .lines()
        .map(|input: &str| {
            let (_, input) = input.split_once(" ").unwrap();
            let (input, joltage) = input.rsplit_once(" ").unwrap();
            let mut joltage = joltage
                .strip_prefix("{")
                .unwrap()
                .strip_suffix("}")
                .unwrap()
                .split(",")
                .map(|joltage| joltage.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            let mut buttons = input
                .split(" ")
                .map(|input| {
                    input
                        .strip_prefix("(")
                        .unwrap()
                        .strip_suffix(")")
                        .unwrap()
                        .split(",")
                        .map(|input| input.parse::<i32>().unwrap())
                        .fold(0i32, |acc, indicator| acc | (1 << indicator))
                })
                .collect::<Vec<_>>();
            buttons.sort_unstable_by_key(|button| button.count_ones());

            let mut vars = good_lp::variables!();
            let vector = vars.add_vector(variable().integer().min(0), buttons.len());
            let objective: Expression = vector.iter().sum();
            let mut pb = vars
                .minimise(objective)
                .using(good_lp::solvers::highs::highs);
            for (i, &joltage) in (0..).zip(&joltage) {
                let buttons = (0..)
                    .zip(&buttons)
                    .filter_map(|(j, &button)| {
                        if button & (1 << i) != 0 {
                            Some(vector[j].into_expression())
                        } else {
                            None
                        }
                    })
                    .reduce(|a, b| a + b)
                    .unwrap();
                pb = pb.with(constraint!(buttons == joltage));
            }
            pb.solve()
                .unwrap()
                .eval(vector.iter().sum::<Expression>())
                .round_ties_even() as i32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            33,
            sum("\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
")
        );
    }
}

fn main() {
    println!(
        "{}",
        sum(&std::io::read_to_string(std::io::stdin()).unwrap())
    );
}
