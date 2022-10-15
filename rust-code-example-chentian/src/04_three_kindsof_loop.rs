fn fib_loop(n: u8) {
    let mut a = 1;
    let mut b = 1;

    let mut i = 2u8;

    loop {
        let c = a + b;
        a = b;
        b = c;
        i += 1;

        println!("Next value is {}", b);

        if i >= n {
            break;
        }
    }
}

fn while_fn(n: u8) {
    let (mut a, mut b, mut i) = (1, 1, 2);

    while i < n {
        let c = a + b;
        a = b;
        b = c;
        i = i + 1;

        println!("Next value is {}", b)
    }
}

// 注意：for循环实现了迭代器，实际上编译器会使用loop循环从迭代器中取值,只至返回的值为None
fn fib_for(n: u8) {
    // Range 中的2..n，包左不包右边,当然可以使用=来包住右边
    let (mut a, mut b) = (1, 1);

    for _i in 2..n {
        let c = a + b;
        a = b;
        b = c;
        println!("Next value is {}", b)
    }
}

fn main() {
    let n: u8 = 8;
    fib_loop(n);
    while_fn(n);
    fib_for(n);

    let arr = [1, 2, 3];
    assert_eq!(arr[..], [1, 2, 3]);
    assert_eq!(arr[0..=1], [1, 2]);
}
