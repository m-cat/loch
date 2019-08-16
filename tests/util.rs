use pretty_assertions::assert_eq;
use std::fmt::Debug;

/// Asserts that the lists contain the same elements, unordered.
pub fn assert_list_eq<T>(list1: &[T], list2: &[T])
where
    T: Clone + Debug + Ord,
{
    let mut list1 = list1.to_vec();
    let mut list2 = list2.to_vec();

    // Sort the input for reliable diffs with pretty-assertions.
    list1.sort();
    list2.sort();

    assert_eq!(list1, list2);
}
