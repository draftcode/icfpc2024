use common;

fn main() {
    println!("{}", hello_world());
}

fn hello_world() -> String {
    format!("{}, world!", common::hello())
}

#[cfg(test)]
mod tests {
    use crate::hello_world;

    #[test]
    fn test_hello() {
        assert_eq!(hello_world(), "Hello, world!");
    }
}
