fn sum(input: &str) -> usize {
    let mut lines = input.lines();
    let mut data = lines
        .next()
        .unwrap()
        .bytes()
        .map(|b| if b == b'S' { 1 } else { 0 })
        .collect::<Vec<_>>();
    for line in input.lines() {
        let mut next = vec![0; data.len()];
        for (i, (b, beams)) in (0usize..).zip(line.bytes().zip(data)) {
            if b == b'^' {
                for i in [i.wrapping_sub(1), i + 1] {
                    if let Some(data) = next.get_mut(i) {
                        *data += beams;
                    }
                }
            } else {
                next[i] += beams;
            }
        }
        data = next;
    }
    data.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            40,
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
