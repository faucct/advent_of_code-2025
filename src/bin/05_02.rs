fn sum(input: &str) -> usize {
    let mut lines = input.lines();
    let mut intervals = Vec::new();
    while let Some(line) = lines.next() {
        if let Some((start, end)) = line.split_once("-") {
            intervals.push(start.parse::<i64>().unwrap()..end.parse::<i64>().unwrap() + 1);
        } else {
            break;
        }
    }
    intervals.sort_by_key(|interval| interval.start);
    let mut sum = 0;
    let mut aggregated = 0..0;
    for interval in intervals {
        if aggregated.end < interval.start {
            sum += aggregated.size_hint().0;
            aggregated = interval;
        } else {
            aggregated.end = aggregated.end.max(interval.end);
        }
    }
    sum + aggregated.size_hint().0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            14,
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
