fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let mut calories: Vec<i32> = input.split("\n\n")
        .into_iter()
        .map(|l| l.lines()
            .into_iter()
            .map(|p| p.parse().unwrap())
            .collect::<Vec<i32>>()
            .iter()
            .sum())
        .collect::<Vec<i32>>();
    let max = *calories.iter().max().unwrap();
    calories.sort_by(|d, b| b.cmp(d));
    let z = &calories[..3].iter().sum::<i32>();

    println!("Max: {max}");
    println!("Max three: {z}");
}