fn sum(reader: impl std::io::BufRead) -> i64 {
    let mut sum = 0;
    for line in reader.lines() {
        for range in line.unwrap().split(",") {
            let (start, end) = range.split_once("-").unwrap();
            let start = start.parse::<i64>().unwrap();
            let end = end.parse::<i64>().unwrap();
            for id in start..=end {
                let tens = 10i64.pow(1 + id.ilog10() / 2);
                if id / tens == id % tens && id.ilog10() % 2 != 0 {
                    sum += id;
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            1227775554,
            sum("\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
".as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}
