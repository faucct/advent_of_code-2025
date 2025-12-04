fn sum(input: &str) -> i32 {
    let mut lines = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.as_bytes().iter().copied().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut count = 0;
    loop {
        let mut quiet = true;
        for i in 0usize..lines.len() {
            for j in 0usize..lines[0].len() {
                if lines[i][j] == b'@'
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
                    quiet = false;
                    lines[i][j] = b'.';
                }
            }
        }
        if quiet {
            return count;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            43,
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
