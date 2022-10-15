#![allow(unused)]

struct A<T>(T);
fn main() {
    // 晚限定,先声明，用的时候再指定
    let a = A::<i32>(3);
    let b = A::<String>("hello".to_string());
}
