fn sum(input: &str) -> i32 {
    let mut lines = input.lines();
    let mut intervals = Vec::new();
    while let Some(line) = lines.next() {
        if let Some((start, end)) = line.split_once("-") {
            intervals.push(start.parse::<i64>().unwrap()..=end.parse().unwrap());
        } else {
            break;
        }
    }
    lines
        .filter(|line| {
            if line.is_empty() {
                return false;
            }
            let line = line.parse::<i64>().unwrap();
            intervals.iter().any(|interval| interval.contains(&line))
        })
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            3,
            sum("\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
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
