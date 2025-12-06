fn sum(input: &str) -> i64 {
    let (input, operations) = input.trim_end_matches("\n").rsplit_once("\n").unwrap();
    let mut sum = 0;
    let mut data = 0;
    let mut operation = 0;
    for (column, &b) in (0..).zip(operations.as_bytes()) {
        if b != b' ' {
            operation = b;
            sum += data;
            data = match b {
                b'+' => 0,
                b'*' => 1,
                _ => panic!(),
            };
        }
        let mut num = 0;
        for row in 0..(input.len() + 1) / (operations.len() + 1) {
            let input = input.as_bytes()[row * (operations.len() + 1) + column];
            if input == b' ' {
                if num != 0 {
                    break;
                }
            } else {
                num = num * 10 + (input - b'0') as i64;
            }
        }
        if num != 0 {
            match operation {
                b'+' => data += num,
                b'*' => data *= num,
                _ => panic!(),
            }
        }
    }
    sum + data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            3263827,
            sum("\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
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
