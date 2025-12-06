fn sum(input: &str) -> i64 {
    let mut lines = input.lines().rev();
    loop {
        let operations = lines.next().unwrap();
        if !operations.is_empty() {
            let operations = operations
                .as_bytes()
                .iter()
                .filter_map(|&b| if b != b' ' { Some(b) } else { None })
                .collect::<Vec<_>>();
            let mut data = operations
                .iter()
                .map(|&operation| match operation {
                    b'+' => 0,
                    b'*' => 1,
                    _ => panic!(),
                })
                .collect::<Vec<_>>();
            for line in lines {
                for ((num, &operation), data) in line
                    .split(" ")
                    .filter(|num| !num.is_empty())
                    .zip(&operations)
                    .zip(&mut data)
                {
                    let num = num.parse::<i64>().unwrap();
                    match operation {
                        b'+' => {
                            *data += num;
                        }
                        b'*' => {
                            *data *= num;
                        }
                        _ => panic!(),
                    }
                }
            }
            return data.into_iter().sum();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            4277556,
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
