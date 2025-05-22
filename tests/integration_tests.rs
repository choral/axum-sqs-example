#[cfg(test)]
#[tokio::test]
async fn my_test() {
    println!("hello world");
}

#[test]
fn add_two_and_two() {
    use axum_sqs_lib::my_math;
    let result = my_math::add_two(2);
    assert_eq!(result, 4);
}
