fn sum(reader: impl std::io::BufRead) -> u64 {
    reader.lines().map(|line| {
        let line = line.unwrap();
        if line.is_empty() {
            return 0;
        }
        let mut max = 0;
        let mut prevs = [0; 11];
        for b in line.into_bytes() {
            let digit = (b - b'0') as u64;
            let mut prevs = prevs.iter_mut();
            let mut prev = prevs.next().unwrap();
            max = max.max(*prev * 10 + digit);
            for next in prevs {
                *prev = (*next * 10 + digit).max(*prev);
                prev = next;
            }
            *prev = digit.max(*prev);
        }
        max
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            3121910778619,
            sum("\
987654321111111
811111111111119
234234234234278
818181911112111
".as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}
