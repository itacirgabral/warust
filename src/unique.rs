fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
where
    T: std::iter::IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
    /*
    let mut eh_primeira = true;
    let mut unique: Vec<T::Item> = Vec::new();

    for s in sequence.into_iter() {
        if eh_primeira {
            println!("ehPrimeiro");
            eh_primeira = false;
            unique.push(s);
        } else {

        };
    };
    Vec::new()
    */
    let v = [10, 40, 30];
    match v.last() {
        None => println!("None"),
        Some(lst) => {
            println!("last {}", lst);
        },
    };
    println!("{:#?}", v);
    let unique: Vec<T::Item> = Vec::new();
    unique
}

#[test]
fn sample_test() {
    assert_eq!(unique_in_order("AAAABBBCCDAABBB".chars()), vec!['A','B','C','D','A','B']);
}