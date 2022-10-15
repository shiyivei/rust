#![allow(unused)]

macro_rules! calculate {
    // eval 1 + 2 $e是宏变量 expr 指类型
    (eval $e:expr) => {{
        {
            let val: usize = $e;
            // 第一个{},将表达式 字符串化，然后打印，第二个是值
            println!("{} = {}",stringify!($e),val);
        }
    }};
}

fn main() {
    calculate! {
        eval 1 + 2
    }

    calculate! {
        eval (1 + 2) * (3 / 1)
    }
}
