// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests2` for hints :)

// 

#[cfg(test)]
mod tests {

    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    #[test]
    fn you_can_assert_eq() {
        assert_eq!(2, add(1,1));
    }
}
