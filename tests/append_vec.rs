use rust_http_web_lib::util::append_vec::append_vec;

#[test]
fn append_element_to_vec_test() {
    let mut vec_a = vec![1, 2];
    let vec_b = vec![3, 4];
    append_vec(&mut vec_a, &vec_b);
    assert_eq!(vec_a, vec![1, 2, 3, 4])
}
