fn sum(input: &str) -> i32 {
    fn rec<'s>(
        paths: &mut std::collections::HashMap<&'s str, i32>,
        graph: &std::collections::HashMap<&'s str, Vec<&'s str>>,
        to: &'s str,
    ) -> i32 {
        if to == "you" {
            return 1;
        }
        if let Some(&paths) = paths.get(to) {
            return paths;
        }
        let mut sum = 0;
        if let Some(from) = graph.get(to) {
            for from in from {
                sum += rec(paths, graph, from);
            }
        }
        paths.insert(to, sum);
        sum
    }
    let mut graph = std::collections::HashMap::new();
    for input in input.trim().lines() {
        let (from, to) = input.split_once(": ").unwrap();
        for to in to.split(" ") {
            graph.entry(to).or_insert(Vec::new()).push(from);
        }
    }
    rec(&mut Default::default(), &graph, "out")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            5,
            sum("\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out")
        );
    }
}

fn main() {
    println!(
        "{}",
        sum(&std::io::read_to_string(std::io::stdin()).unwrap())
    );
}
