#
fn split_assert() {
        let v: Vec<&str> = "Mary had a little lambda".splitn(3, ' ').collect();
        assert_eq!(v, ["Mary", "had", "a little lambda"]);
}