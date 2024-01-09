// tests2.rs
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a
// hint.


fn makes_string() -> String {
    "T".to_string()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn you_can_assert_eq() {
        let a = makes_string();
        assert_eq!(String::from("T"), a);
    }
}
