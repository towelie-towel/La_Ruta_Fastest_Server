fn main() {
    println!("Hello, world!");
    file
        .lines()
        .enumerate()
        .filter(|(idx, _)| idx % 2 == 0)
        .for_each(|( _, line)| println("{}", line));
}
