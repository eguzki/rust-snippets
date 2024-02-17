fn main() {
    let mut v1 = vec![1, 2, 3];

    // the .iter_mut() method creates an iterator,
    // v1_iter which borrows value and can mutate it.
    let mut v1_iter = v1.iter_mut();

    // access the first item and multiple it by 2
    let item1 = v1_iter.next().unwrap();
    *item1 = *item1 * 2;

    // access the second item and multiple it by 2
    let item2 = v1_iter.next().unwrap();
    *item2 = *item2 * 2;

    // access the third item and multiple it by 2
    let item3 = v1_iter.next().unwrap();
    *item3 = *item3 * 2;

    // end of the iteration
    assert_eq!(v1_iter.next(), None);

    // this will print out [2,4,6]
    dbg!(v1);
}
