fn main() {
    let name = "Aristella";
    let times = 3;
    elaborate_hello(name, times);
}

fn elaborate_hello(name: &str, times: usize) {
    for i in 1..=times {
        println!("{}. Hello, {}! Welcome to the world of Rust.", i, name);
    }
}
