mod simple;
use simple::*;

#[test]
fn test_hello_world() {
	assert_eq!("Hello, World!", hello());
}