fn sum(input: &str) -> i32 {
    let lines = input
        .lines()
        .map(|line| line.as_bytes())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();
    let mut count = 0;
    for (i, &line) in (0usize..).zip(&lines) {
        for (j, &cell) in (0usize..).zip(line) {
            if cell == b'@'
                && [i.wrapping_sub(1), i, i.wrapping_add(1)]
                    .into_iter()
                    .flat_map(|i| {
                        let lines = &lines;
                        [j.wrapping_sub(1), j, j.wrapping_add(1)]
                            .into_iter()
                            .filter(move |&j| {
                                Some(&b'@') == lines.get(i).and_then(|line| line.get(j))
                            })
                    })
                    .skip(4)
                    .all(|_| false)
            {
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
            13,
            sum("\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
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
