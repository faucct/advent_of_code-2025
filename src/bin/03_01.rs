fn sum(reader: impl std::io::BufRead) -> i32 {
    reader.lines().map(|line| {
        let line = line.unwrap();
        if line.is_empty() {
            return 0;
        }
        let mut max = 0;
        let mut prev = 0;
        for b in line.into_bytes() {
            let digit = b - b'0';
            max = max.max(prev * 10 + digit);
            prev = prev.max(digit);
        }
        max as i32
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            357,
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
