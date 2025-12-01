fn sum(reader: impl std::io::BufRead) -> i32 {
    let mut count = 0;
    let mut dial = 50;
    for line in reader.lines() {
        let line = line.unwrap();
        if !line.is_empty() {
            let mut line = line.chars();
            if line.next().unwrap() == 'R' {
                dial += line.as_str().parse::<i32>().unwrap();
            } else {
                dial += 100 - line.as_str().parse::<i32>().unwrap();
            }
            dial %= 100;
            if dial == 0 {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            3,
            sum("\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
".as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}
