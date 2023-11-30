#![crate_name = "d01"]
fn main() {
    println!("Hello, world!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        main()
    }
}
