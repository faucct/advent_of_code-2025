fn sum(input: &str) -> i64 {
    let corners = input
        .trim()
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
        })
        .collect::<Vec<_>>();
    let mut max = 0;
    for (i, &a) in (0..).zip(&corners) {
        for &b in &corners[..i] {
            max = max.max((a.0.max(b.0) - a.0.min(b.0) + 1) * (a.1.max(b.1) - a.1.min(b.1) + 1));
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            50,
            sum("\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
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
