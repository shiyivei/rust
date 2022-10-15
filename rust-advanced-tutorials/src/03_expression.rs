#[feature(min_const_generics)]
use core::mem::MaybeUninit;
pub struct ArrayVec<T, const N: usize> {
    items: [MaybeUninit<T>; N],
    length: usize,
}

#[allow(unused)]
fn main() {
    // 整个表达式返回的都是单元类型 ()
    for i in 1..102 {
        if i % 15 == 0 {
            println!("FizzBuzz")
        } else if i % 3 == 0 {
            println!("Fizz")
        } else if i % 5 == 0 {
            println!("Buzz")
        } else {
            println!("{}", i)
        }
    }

    println!("--------------------------------");

    // match 也是表达式返回的是元组
    for i in 1..102 {
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Buzz"),
            (_, 0) => println!("Fizz"),
            (_, _) => println!("{}", i),
        }
    }

    //编译期优化和常量传播
    const X: u32 = 3 + 4; //CTFE
    let x: u32 = 4 + 3; // 不是CTFE，但是可能会被常量传播优化，因为它不在常量上下文
}
