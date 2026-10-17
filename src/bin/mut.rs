fn main() {
    let immutable = 1;
    let mut mutable = 12;

    println!("Mutable value before mutating {} ", mutable); // 12
    mutable += 12;
    println!("Mutable value after mutation {}", mutable); // 24
    immutable += 1; // error
}
