fn main() {
    //将数字存放到堆内存上，此时x就是装箱类型，是个引用，因此可以解引用
    let x: Box<i32> = Box::new(42);
    //解引用
    let y = *x;

    assert_eq!(y, 42)

    //Box实现了Drop类型
}
