fn main() {
    //路径的第一种用法
    pub mod a {
        fn foo() {
            println!("a")
        }

        pub mod b {
            pub mod c {
                pub fn foo() {
                    super::super::foo();
                    self::super::super::foo();
                }
            }
        }
    }

    //模块调用
    a::b::c::foo();

    //路径的第二种用法 在trait中
    struct S;

    impl S {
        fn f() {
            println!("S")
        }
    }

    trait T1 {
        fn f() {
            println!("T1 f")
        }
    }

    impl T1 for S {}

    trait T2 {
        fn f() {
            println!("T2 f")
        }
    }

    impl T2 for S {}

    S::f(); //直接调用关联函数
            //完全限定无歧义调用,将结构体转换为trait名称，就可以通过trait调用里面的函数了
    <S as T1>::f();
    <S as T2>::f();

    //路径的第三种用法,在泛型函数中使用 :: 是路径分割符号,turbofish 操作符
    //泛型函数是collect使用路径分割符 + 类型构造符<> 里面再填上具体类型 可以是Vec<i32>,也可以使用通配符_,让编译器推断
    let x = (0..10).collect::<Vec<_>>();
    //给数组指定了具体类型u8
    let y = Vec::<i32>::with_capacity(1024); //with_capacity是Vec的一个方法，而vec本身可以通过路径分割符的形式指定类型
    println!("the value of x is {:?},y is {:?}", x, y)
}
