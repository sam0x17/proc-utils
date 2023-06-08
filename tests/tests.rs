use proc_utils::*;

#[test]
fn test_item_suppression() {
    #[suppress_item]
    fn invalid_rust() {
        return nonexistent_value;
    }
}
