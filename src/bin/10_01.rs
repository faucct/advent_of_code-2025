fn sum(input: &str) -> i32 {
    input
        .trim()
        .lines()
        .map(|input| {
            let (indicator, input) = input.split_once(" ").unwrap();
            let indicator = indicator
                .strip_prefix("[")
                .unwrap()
                .strip_suffix("]")
                .unwrap()
                .as_bytes()
                .iter()
                .enumerate()
                .fold(0, |acc, b| acc | if *b.1 == b'#' { 1 << b.0 } else { 0 });
            let (input, _) = input.rsplit_once(" ").unwrap();
            let buttons = input
                .split(" ")
                .map(|input| {
                    input
                        .strip_prefix("(")
                        .unwrap()
                        .strip_suffix(")")
                        .unwrap()
                        .split(",")
                        .map(|input| input.parse::<i32>().unwrap())
                        .fold(0, |acc, indicator| acc | (1 << indicator))
                })
                .collect::<Vec<_>>();
            let mut pushes = 1;
            let mut visited = std::collections::HashSet::new();
            visited.insert(indicator);
            let mut queue = std::collections::VecDeque::new();
            let mut queue_len = 1;
            queue.push_back(indicator);
            while let Some(indicator) = queue.pop_front() {
                for &button in &buttons {
                    let indicator = indicator ^ button;
                    if indicator == 0 {
                        return pushes;
                    }
                    if visited.insert(indicator) {
                        queue.push_back(indicator);
                    }
                }
                queue_len -= 1;
                if queue_len == 0 {
                    pushes += 1;
                    queue_len = queue.len();
                }
            }
            panic!()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            7,
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
