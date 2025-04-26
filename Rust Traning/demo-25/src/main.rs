fn drop_trait() {
    println!("\n---Drop Trait---");
    let d = Drop {
        name: String::from("Drop"),
    };
    println!("d: {}", d.name);
}

fn main() {
    println!("\n---------------Traits-----------------");

    drop_trait();
}
