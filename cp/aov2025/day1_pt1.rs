fn main() {
    let mut res: i32 = 0;
    let mut pointer: i32 = 50;
    for line in include_str!("./day1.txt").lines() {
        match line.chars().next() {
            Some('L') => {
                let n: i32 = line[1..].parse().unwrap();
                pointer = (pointer - n).rem_euclid(100);
            }
            Some('R') => {
                let n: i32 = line[1..].parse().unwrap();
                pointer = (pointer + n).rem_euclid(100);
            }
            _ => (),
        }
        if pointer == 0 || pointer == 100 {
            res += 1;
        }
    }
    println!("{}", res);
}
