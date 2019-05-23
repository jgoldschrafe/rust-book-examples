fn main() {
    let s = no_dangle();
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}