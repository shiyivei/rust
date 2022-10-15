//定义一个类型别名（类型）
type RGB = (i16, i16, i16);

//定义函数
fn color(_c: &str) -> RGB {
    (1, 1, 1)
}

//定义另一个函数，函数的参数是一个函数
fn show(c: fn(&str) -> RGB) {
    println!("{:?}", c("black"));
}

fn main() {
    //将函数作为值赋值给变量
    let rgb = color;
    let c: fn(&str) -> RGB = rgb;
    //这里被转换为了函数指针类型
    show(rgb);

    // println!("{:?}", std::mem::size_of_val(&rgb));
    // println!("{:?}", std::mem::size_of_val(&c));

    //定义了实现 ‘Fn(&str) -> RGB’ trait 的闭包类型

    let d = |s: &str| (1, 2, 3);
    show(d);
}
