use std::env;

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);

    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }

        m = m % n;
    }
    n
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: cargo run <number1> <number2>");
        std::process::exit(1);
    }

    // validate numbers
    let n: u64 = args[1].parse().expect("Positive number required");
    let m: u64 = args[2].parse().expect("Positive number required");

    let result = gcd(n, m);
    println!("The GCD of {} and {} is {}", n, m, result);
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2*3*5*11*17, 3*7*11*13*19), 3 * 11);
}