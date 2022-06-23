use std::vec;

fn vec_of_results() {
    let inputs: Vec<i64> = vec![0, 1, 2, 3, 4, 512];
    let result: Vec<Result<u8, _>> = inputs.into_iter().map(|v| <u8>::try_from(v)).collect();
    println!("{:?}", result);
}

fn results_with_vec() {
    let inputs: Vec<i64> = vec![0, 1, 2, 3, 4];
    let result = inputs
        .into_iter()
        .map(|v| <u8>::try_from(v))
        // Result's FromIterator trait implementation allows going from
        // vec of results to a result with a vec (if there is no error)
        .collect::<Result<Vec<_>, _>>();
    println!("{:?}", result);
}

fn main() {
    vec_of_results();
    results_with_vec();
}
