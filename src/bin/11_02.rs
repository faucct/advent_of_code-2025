fn sum(input: &str) -> i64 {
    fn rec<'s>(
        paths: &mut std::collections::HashMap<&'s str, [i64; 4]>,
        graph: &std::collections::HashMap<&'s str, Vec<&'s str>>,
        to: &'s str,
    ) -> [i64; 4] {
        if to == "svr" {
            return [1, 0, 0, 0];
        }
        if let Some(&paths) = paths.get(to) {
            return paths;
        }
        let mut sums = [0; 4];
        if let Some(from) = graph.get(to) {
            for from in from {
                for (sum, rec) in sums.iter_mut().zip(rec(paths, graph, from)) {
                    *sum += rec;
                }
            }
        }
        match to {
            "fft" => {
                sums[3] += std::mem::take(&mut sums[2]);
                sums[1] += std::mem::take(&mut sums[0]);
            }
            "dac" => {
                sums[3] += std::mem::take(&mut sums[1]);
                sums[2] += std::mem::take(&mut sums[0]);
            }
            _ => {}
        }
        paths.insert(to, sums);
        sums
    }
    let mut graph = std::collections::HashMap::new();
    for input in input.trim().lines() {
        let (from, to) = input.split_once(": ").unwrap();
        for to in to.split(" ") {
            graph.entry(to).or_insert(Vec::new()).push(from);
        }
    }
    rec(&mut Default::default(), &graph, "out")[3]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            2,
            sum("\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out")
        );
    }
}

fn main() {
    println!(
        "{}",
        sum(&std::io::read_to_string(std::io::stdin()).unwrap())
    );
}
