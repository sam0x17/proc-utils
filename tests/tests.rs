use proc_utils::*;

#[test]
fn test_item_suppression() {
    #[suppress_item]
    fn invalid_rust() {
        return nonexistent_value;
    }
}

#[test]
fn test_overwrite_with() {
    #[overwrite_with {
		fn hello_world() -> usize {
			return 3;
		}
	}]
    fn invalid_rust() {
        return nonexistent_value;
    }
    assert_eq!(hello_world(), 3);
}
