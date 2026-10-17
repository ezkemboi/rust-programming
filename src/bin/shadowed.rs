fn main() {
    let shadowed_binding = 1;
    {
        println!("Shadowed binding inner before change {}", shadowed_binding);
        let shadowed_binding = 2;
        println!("Shaowed binding inner after change {}", shadowed_binding);
    }

    println!("Shadowed binding outer before change {}", shadowed_binding);
    let shadowed_binding = 3;
    println!("Shadowed binding outer after change {}", shadowed_binding);
}
