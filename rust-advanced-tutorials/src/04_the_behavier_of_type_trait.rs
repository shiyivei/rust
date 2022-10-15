//定义类型
struct Point(i32, i32);

//定义trait,使用trait定义公共的行为
pub trait FromStr {
    type Err;
    fn from_str(s: &str) -> Result<Self, Self::Err>;
}

//为类型实现trait（也就是实现trait中定义的方法）
impl FromStr for u32 {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //do something
    }
}

//为多个类型实现trait（也就是实现trait中定义的方法）
impl FromStr for Point {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //do something
    }
}

//为类型实现方法,使用泛型约束，这会让parse只接受实现了FromStr的类型
impl str {
    pub fn parse<F: FromStr>(&self) -> Result<F, <F as FromStr>::Err> {
        F::from_str(self);
    }
}

fn main() {
    //将字符串转为u32
    let four: u32 = "4".parse().unwrap();
    assert_eq!(4, four);
    println!("{}", four);

    //将字符串转为自定义类型，也是要调用转换字符串的方法，引入trait抽象一组相同的行为
    // let p = "(1,2)".parse::<Point>();
    // assert_eq!(p.unwrap(), Point(1, 2));

    //trait一致性规则
}
