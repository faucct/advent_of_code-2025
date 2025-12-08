fn sum(connections: i32, input: &str) -> usize {
    use std::cell::Cell;
    fn root(
        components: &Vec<(Cell<usize>, Cell<usize>)>,
        node: usize,
    ) -> &(Cell<usize>, Cell<usize>) {
        let parent = &components[node];
        if parent.0.get() == node {
            parent
        } else {
            let root = root(components, parent.0.get());
            parent.0.set(root.0.get());
            root
        }
    }
    let jukeboxes = input
        .trim()
        .lines()
        .map(|line| {
            let mut input = line.split(",").map(|input| input.parse::<i64>().unwrap());
            (
                input.next().unwrap(),
                input.next().unwrap(),
                input.next().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    let mut binary_heap =
        std::collections::BinaryHeap::with_capacity(jukeboxes.len() * (jukeboxes.len() - 1) / 2);
    for (i, b) in (0..).zip(&jukeboxes) {
        for (j, &a) in (0..).zip(&jukeboxes[..i]) {
            let x = a.0 - b.0;
            let y = a.1 - b.1;
            let z = a.2 - b.2;
            binary_heap.push((-(x * x + y * y + z * z), i, j));
        }
    }
    let components = (0..jukeboxes.len())
        .map(|jukebox| (Cell::new(jukebox), Cell::new(1)))
        .collect::<Vec<_>>();
    for _ in 0..connections {
        if let Some((_, i, j)) = binary_heap.pop() {
            let i = root(&components, i);
            let j = root(&components, j);
            if i.0.get() != j.0.get() {
                let (min, max) = if i.1.get() < j.1.get() {
                    (i, j)
                } else {
                    (j, i)
                };
                min.0.set(max.0.get());
                max.1.set(max.1.get() + min.1.take());
            }
        } else {
            break;
        }
    }
    let mut sizes = [0; 4];
    for (i, component) in (0..).zip(components) {
        if component.0.get() == i {
            sizes[0] = component.1.get();
            sizes.sort_unstable();
        }
    }
    sizes[0] = 1;
    sizes.iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            40,
            sum(
                10,
                "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"
            )
        );
    }
}

fn main() {
    println!(
        "{}",
        sum(1000, &std::io::read_to_string(std::io::stdin()).unwrap())
    );
}
