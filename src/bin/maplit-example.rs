use maplit::*;
use std::collections::*;

fn main() {
    assert_eq!(hashset! { "a", "b" }, ["a", "b"].into_iter().collect());
    assert_eq!(btreeset! { "a", "b" }, ["a", "b"].into_iter().collect());
    assert_eq!(
        hashmap! { "a" => 1, "b" => 2 },
        [("a", 1), ("b", 2)]
            .into_iter()
            .collect::<HashMap<&str, u32>>()
    );
    assert_eq!(
        btreemap! { "a" => 1, "b" => 2 },
        [("a", 1), ("b", 2)]
            .into_iter()
            .collect::<BTreeMap<&str, u32>>()
    );
}
