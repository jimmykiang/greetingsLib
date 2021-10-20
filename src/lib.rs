pub fn hello() -> String {
    ("Testing library").to_string()
}

// 01. Tests for `hello()`
#[test] // Indicates that this is a test function
fn test_hello() {
    assert_eq!(hello(), "Testing library");
}

// 02. Tests for `hello()`, Idiomatic way
#[cfg(test)] // Only compiles when running tests
mod tests { // Separates tests from code
use super::hello; // Import root `hello()` function

    #[test]
    fn test_hello() {
        assert_eq!(hello(), "Testing library");
    }
}
