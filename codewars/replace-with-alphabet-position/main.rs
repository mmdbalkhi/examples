const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn alphabet_position(text: &str) -> String {
    let mut ret = String::new();
    for c in text.to_lowercase().chars() {
        let pos = match ALPHABET.find(c) {
            Some(pose) => pos + 1,
            _ => 0,
        };
        if pos == 0 {
            continue;
        }
        if !ret.is_empty() {
            ret.push(' ');
        }
        ret.push_str(&pos.to_string());
    }
    ret
}

fn main() {
    println!(
        "{}",
        alphabet_position("The sunset sets at twelve o' clock.")
    );
}
#[test]
fn returns_expected() {
    assert_eq!(
        alphabet_position("The sunset sets at twelve o' clock."),
        "20 8 5 19 21 14 19 5 20 19 5 20 19 1 20 20 23 5 12 22 5 15 3 12 15 3 11".to_string()
    );
    assert_eq!(
        alphabet_position("The narwhal bacons at midnight."),
        "20 8 5 14 1 18 23 8 1 12 2 1 3 15 14 19 1 20 13 9 4 14 9 7 8 20".to_string()
    );
}
