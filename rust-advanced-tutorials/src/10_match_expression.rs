struct Point {
    x: i32,
    y: i32,
}

fn sum(x: String, ref y: String) -> String {
    x + y
}

//match
fn check_optional(opt: Option<i32>) {
    match opt {
        Some(p) => println!("has value {}", p),
        None => println!("has no value"),
    }
}

// fn handle_result(res: i32) -> Result<i32, dyn Error> {
//     do_something(res)?;

//     //问好等价于
//     match do_something(res) {
//         Ok(o) => Ok(o),
//         Err(e) = return SomeError(e)
//     }
// }

fn main() {
    let (a, b) = (1, 2);

    let Point { x, y } = Point { x: 3, y: 4 };

    assert_eq!(1, a);
    assert_eq!(2, b);
    assert_eq!(3, x);
    assert_eq!(4, y);

    // 函数与闭包参数

    let s = sum("1".to_owned(), "2".to_owned());
    assert_eq!(s, "12".to_owned());

    // 理解 ref
    let a = 42;
    let ref b = a;

    let c = &a;
    assert_eq!(b, c);

    let mut a = [1, 2, 3];
    let ref mut b = a;
    b[0] = 0;

    //说明 ref 左边 等价于 &右边

    assert_eq!(a, [0, 2, 3]);

    //if let
    let x = &Some(1);

    if let Some(y) = x {
        y;
    }
}
