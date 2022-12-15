
fn main() {
    let mut max = 0;
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    for line in input.split("\n\n") {
        let mut sum = 0;
        for l in line.lines() {
            let n: i32 = l.parse().unwrap();
            sum += n;
        }
        max = if sum > max { sum } else { max };
    }
    println!("Max: {max}")
}