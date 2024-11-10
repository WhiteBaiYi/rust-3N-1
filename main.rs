use std::io;

fn main() {
    let mut input = String::new();

    println!("Please type a number to continue:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Parse the input into an integer (u128 for larger numbers)
    let mut n: u128 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid integer.");
            return; // Exit the program if input is invalid
        }
    };

    println!("Starting number: {}", n);

    // Start the Collatz sequence using a while loop
    while n != 1 {
        println!("{}", n); // Print the current value of n

        // Apply the Collatz transformation
        if n % 2 == 0 {
            n /= 2; // If n is even, divide by 2
        } else {
            n = 3 * n + 1; // If n is odd, multiply by 3 and add 1
        }
    }

    // Once the loop exits, n must be 1
    println!("This number matches 3N+1"); // Collatz sequence reached 1
}

