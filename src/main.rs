fn main() {
    println!("Let's find some primes!");

    let mut counter: u128 = 3;

    let mut primes: Vec<u128> = vec![2];

    loop {
        // println!("Testing {counter}");

        let mut new_primes: Vec<u128> = Vec::new();

        'for_loop: for prime in &primes {
            // println!("Testing if {counter} is divisible by {prime}");
            if &counter % prime == 0 {
                // println!("{counter} is divisible by {prime}");
                break 'for_loop;
            } else if Some(prime) == primes.last() {
                println!("{counter} is prime");
                new_primes.push(counter);
                break 'for_loop;
            }
        }

        primes.extend(new_primes);
        // println!("counter is {counter} outside");
        // println!("{counter} is not prime");
        
        counter += 1;
    }
}
