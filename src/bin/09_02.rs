fn sum(input: &str) -> u64 {
    let mut corners = input
        .trim()
        .lines()
        .map(|line| {
            let mut line = line.split(",");
            let mut corner =
                std::array::from_fn::<_, 2, _>(|_| line.next().unwrap().parse::<u64>().unwrap());
            corner.reverse();
            corner
        })
        .collect::<Vec<_>>();
    let grid = std::array::from_fn::<_, 2, _>(|i| {
        let mut points = Vec::from_iter(
            corners
                .iter()
                .flat_map(|&corner| [corner[i] - 1, corner[i]])
                .collect::<std::collections::HashSet<_>>(),
        );
        points.sort_unstable();
        let grid = points
            .iter()
            .copied()
            .zip(0..)
            .collect::<std::collections::HashMap<_, _>>();
        for corner in &mut corners {
            corner[i] = grid[&corner[i]];
        }
        points
    });

    let mut map = vec![vec![b'.'; grid[1].len()]; grid[0].len()];
    let mut prev = *corners.last().unwrap();
    for &next in &corners {
        map[next[0] as usize][next[1] as usize] = b'#';
        if next[0] == prev[0] {
            for j in prev[1].min(next[1]) + 1..prev[1].max(next[1]) {
                map[next[0] as usize][j as usize] = b'X';
            }
        } else {
            for i in prev[0].min(next[0]) + 1..prev[0].max(next[0]) {
                map[i as usize][next[1] as usize] = b'X';
            }
        }
        prev = next;
    }

    for row in &mut map {
        #[derive(PartialEq)]
        enum State {
            Outside,
            Entering,
            Inside,
            Exiting,
        }
        let mut state = State::Outside;
        for cell in row {
            *cell = if match *cell as u8 {
                b'.' => State::Inside == state,
                b'#' => {
                    state = match state {
                        State::Inside => State::Exiting,
                        State::Entering => State::Inside,
                        State::Exiting => State::Outside,
                        State::Outside => State::Entering,
                    };
                    true
                }
                b'X' => {
                    state = match state {
                        State::Inside => State::Outside,
                        State::Outside => State::Inside,
                        state => state,
                    };
                    true
                }
                _ => panic!(),
            } {
                b'X'
            } else {
                b'.'
            };
        }
    }

    let mut grid_prefix = Vec::<Vec<u64>>::with_capacity(grid[0].len());
    for i in 0..grid[0].len() {
        let mut row_prefix = Vec::with_capacity(grid[0].len());
        for j in 0..grid[1].len() {
            row_prefix.push(
                row_prefix.last().unwrap_or(&0)
                    + if let Some(prev) = grid_prefix.last() {
                        prev[j] - *prev.get(j.wrapping_sub(1)).unwrap_or(&0)
                    } else {
                        0
                    }
                    + if map[i][j] == b'X' { 1 } else { 0 },
            );
        }
        grid_prefix.push(row_prefix);
    }
    let mut answer = 0;
    for (i, &a) in (0..).zip(&corners) {
        for &b in &corners[..i] {
            let min = std::array::from_fn::<_, 2, _>(|i| a[i].min(b[i]) - 1);
            let max = std::array::from_fn::<_, 2, _>(|i| a[i].max(b[i]));
            if grid_prefix[max[0] as usize][max[1] as usize]
                + grid_prefix[min[0] as usize][min[1] as usize]
                - grid_prefix[max[0] as usize][min[1] as usize]
                - grid_prefix[min[0] as usize][max[1] as usize]
                == (max[0] - min[0]) * (max[1] - min[1])
            {
                answer = answer.max(
                    (grid[0][max[0] as usize] - grid[0][min[0] as usize])
                        * (grid[1][max[1] as usize] - grid[1][min[1] as usize]),
                );
            }
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            24,
            sum("\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
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
