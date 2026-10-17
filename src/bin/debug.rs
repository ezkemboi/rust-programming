#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    let name = "Ezrqn";
    let age = 30;
    let ezrqn = Person { name, age };

    println!("{:#?}", ezrqn)
}
