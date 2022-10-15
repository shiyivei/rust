fn main() {
    //未捕捉到环境变量 ,所有权，实现FnOnce
    let c1 = || println!("hello");
    c1();

    //可修改环境变量，可变借用，GnMut
    let mut arr = [1, 2, 3];
    let mut c2 = |i| {
        arr[0] = i;
        println!("{:?}", arr);
    };
    c2(10);

    // 未修改环境变量，不可变借用，实现Fn

    let answer = 42;

    let c3 = || println!("{:?}", answer);

    c3();
}
