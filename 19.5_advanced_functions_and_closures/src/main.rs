enum Status {
    Value(u32),
    Stop,
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    let list_of_statuses: Vec<Status> =
        (0u32..20)
        .map(Status::Value)
        .collect();
}
