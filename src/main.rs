fn solve() {}

fn main() {
    std::thread::Builder::new()
        .name("solve".into())
        .stack_size(256 * 1024 * 1024)
        .spawn(solve)
        .unwrap()
        .join()
        .unwrap();
}
