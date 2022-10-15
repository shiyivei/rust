mod lib {

    pub fn pi() -> f64 {
        3.1415926
    }

    pub fn not_pi() {
        3.1415926;
    }
}

/// Rust中，函数是一等公民

fn apply(value: i32, f: fn(i32) -> i32) -> i32 {
    f(value)
}

fn square(value: i32) -> i32 {
    value * value
}

fn cube(value: i32) -> i32 {
    value * value * value
}

fn main() {
    let is_pi = lib::pi();
    let is_uint1 = lib::not_pi();

    let is_uint2 = {
        lib::pi();
    };

    println!(
        "is_pi:{:?},is_uint1:{:?},is_uint2:{:?}",
        is_pi, is_uint1, is_uint2
    );

    //有参数的函数作为值
    println!("apply square {}", apply(2, square));
    println!("apply cube {}", apply(2, cube));
}
