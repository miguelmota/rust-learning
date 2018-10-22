// debug makes it printable
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    let name = "Bob";
    let age = 27;
    let bob = Person { name, age };

    // pretty print, hashtag will indent output
    println!("{:#?}", bob) // Person { name: "Bob", age: 27 }
}
