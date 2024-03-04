use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let vec = line
        .trim()
        .split(' ')
        .flat_map(&str::parse::<i32>)
        .collect::<Vec<_>>();

    println!("{}", vec[0] - vec[1]);
}
