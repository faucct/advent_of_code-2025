fn sum(input: &str) -> usize {
    let mut lines = input.lines();
    let mut splits = 0;
    let mut data = lines
        .next()
        .unwrap()
        .bytes()
        .map(|b| b == b'S')
        .collect::<Vec<_>>();
    for line in input.lines() {
        let mut next = vec![false; data.len()];
        for (i, (b, beam)) in (0usize..).zip(line.bytes().zip(data)) {
            if beam {
                if b == b'^' {
                    splits += 1;
                    for i in [i.wrapping_sub(1), i + 1] {
                        if let Some(data) = next.get_mut(i) {
                            *data = true;
                        }
                    }
                } else {
                    next[i] = true;
                }
            }
        }
        data = next;
    }
    splits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            21,
            sum("\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
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
