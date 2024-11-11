use std::io;

fn main() {
    let mut n = String::new();

    println!("Enter a number for n!");

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line.");

    let n: u128 = n.trim().parse()
        .expect("Must be an integer");

    let n = if n <= 1 {
        n
    } else {
        let mut a = 0;
        let mut b = 1;
        for _ in 2..=n {
            let c = a + b;
            a = b;
            b = c;
        }
        b
    };

    println!("The Fibonacci you want is {n}.");
}
