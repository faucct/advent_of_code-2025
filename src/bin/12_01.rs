fn sum(input: &str) -> i32 {
    let (input, regions) = input.trim().rsplit_once("\n\n").unwrap();
    let presents = input.split("\n\n").map(|present| {
        let input = present.split_once(":\n").unwrap().1;
        input.as_bytes().iter().map(|&b| if b == b'#' { 1 } else { 0 }).sum::<i32>()
    }).collect::<Vec<_>>();
    regions.lines().map(|region| {
        let (shape, quantities) = region.split_once(": ").unwrap();
        let mut shape = shape.split("x").map(|dimension| dimension.parse::<i32>().unwrap());
        let shape = std::array::from_fn::<_, 2, _>(|_| shape.next().unwrap());
        let quantities = quantities
            .split(" ")
            .map(|quantity| quantity.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let mut area = shape[0] * shape[1];
        let mut squares = (shape[0] / 3) * (shape[1] / 3);
        for (&quantity, &present) in quantities.iter().zip(&presents) {
            squares -= quantity;
            area -= quantity * present;
        }
        if area < 0 {
            0
        } else if squares >= 0 {
            1
        } else {
            panic!("{region}");
        }
    }).sum()
}

fn main() {
    println!(
        "{}",
        sum(&std::io::read_to_string(std::io::stdin()).unwrap())
    );
}
