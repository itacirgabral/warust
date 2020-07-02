fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
where
    T: std::iter::IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
    let mut unique: Vec<T::Item> = Vec::new();
    for s in sequence.into_iter() {
        match unique.last() {
            None => unique.push(s),
            Some(lst) => {
                if s != *lst {
                    unique.push(s);
                }
            },
        }
    }
    unique
}

#[test]
fn sample_test() {
    assert_eq!(unique_in_order("AAAABBBCCDAABBB".chars()), vec!['A','B','C','D','A','B']);
}