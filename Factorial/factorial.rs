// lean rust!
// find number's Factorial

fn main() {
        println!("{}", factorial(4));
        println!("{}", functional_factorial(4));
}

fn factorial(num:u32) -> u32 {

        let mut rot:u32 = 1;

        for i in 1..num+1 {
            rot *= i;
        }
        rot
}

fn functional_factorial(num:u32) -> u32
{
    return num*(num-1);
}