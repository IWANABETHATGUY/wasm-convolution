fn main() {
    let a = match 5 / 2 {
        a @ 1..=10 => a,
        std::i32::MIN..=2 => 2,
        11..=std::i32::MAX => 32,
    };
    println!("{}", a);
}
