fn compute(input: &str) -> i64 {
    let mut count: i64 = 0;
    let mut pos: i32 = 50;

    for line in input.lines() {
        let line = line.trim();

        let direction = &line[0..1];
        let mut num: i32 = line[1..].parse().unwrap();

        count += (num / 100) as i64;
        num %= 100;

        if num != 0 {
            let prev = pos;
            if direction == "L" {
                pos -= num;
                if pos <= 0 && 0 < prev {
                    count += 1;
                }
            } else {
                pos += num;
                if pos >= 100 {
                    count += 1;
                }
            }
            pos = pos.rem_euclid(100);
        }
    }
    count
}

fn main() {
    const INPUT: &str = include_str!("./day1.txt");
    println!("{}", compute(INPUT));
}

#[test]
fn test() {
    const INPUT: &str = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
    assert_eq!(compute(INPUT), 6);
}
