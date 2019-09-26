#[no_mangle]
pub extern fn add(x: u32, y: u32) -> u32 {
    x + y
}
#[no_mangle]
pub extern fn fib(mut v: Vec<i32>, n: usize, i: usize) -> Vec<i32> {
    match i < n {
        true => {
            let tmp = v[i-1] + v[i-2];
            v.push(tmp);
            return fib(v, n, i+1);
        }
        _ => {
            return v;
        }
    }
}
#[no_mangle]
pub extern fn fib_test() -> usize {
    let mut v = vec![1,1];
    let n = 12;
    v = fib(v, n, 2);
    assert_eq!(v, vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144]);
    return v.len()
}
