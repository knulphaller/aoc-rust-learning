fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let a: Vec<i32> = input.split("\n\n")
        .into_iter()
        .map(|l| l.lines()
            .into_iter()
            .map(|p| p.parse().unwrap())
            .collect::<Vec<i32>>()
            .iter()
            .sum())
        .collect::<Vec<i32>>();

    let max = &a.iter().max().unwrap();
    println!("Max: {max}")
}