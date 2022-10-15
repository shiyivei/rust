# 1 Rust语言核心难点

张汉东老师的《极客时间》

1. 所有权机制

2. 借用和生命周期

3. 类型系统与trait

4. 突破抽象范式

5. Unsafe Rust

# 2 Rust 学习注意事项

1. 从整体出发，不要陷入细节中
2. Rust语言的层次：所有权系统 + 编程范式 —> 类型系统 —>内存管理
3. 和已知的知识建立联系，如所有权是为了解决安全问题
4. 学会阅读源码，从源码中学习
5. 通过主题式阅读填补知识空白，如对并发安全
6. 时刻把握Rust设计哲学
7. 有意识地构建Rust语言的心智模型
8. 多分享、多提问、多交流
9. 为开源项目做贡献
10. 阅读《Rust编程之道》

# 3 Rust语言概览

## 3.1 Rust 来自哪里

内存安全为第一准则

注重并发安全，避免数据竞争

持续提升性能

保持语言的高度一致性

语言必须有可见的实用性

注重开发体验和学习体验

现代化语言特性

拥抱开源社区

## 3.2 Rust是什么

### 3.2.1 新时代的C语言，是一门通用型语言

### 3.2.2 Rust的内存安全方案针对的是C语言的不足

1. 禁止对空指针和悬垂指针进行解引用

2. 读取未初始化的内存

3. 缓冲区溢出

4. 非法释放已经释放或未分配的指针

注意：内存泄漏不是Rust承诺的内存安全范围

### 3.2.3 安全且无缝沟通C语言

1. 通过C-ABI零成本和C语言打交道
2. 划分了Safe Rust和 Unsafe Rust

### 3.2.4 Rust是混合范式的“面向过程”式的编程语言

1. Rust包含了面向对象（OOP）、函数式（FP）和泛型编程范式
2. OOP和FP范式在Rust语言中是作为语言特性而存在，并非是抽象方式
3. Rust让你专注于解决问题本身，而不受编程范式思想框架的干扰

### 3.2.5 和C语言，担负时代使命

1. 操作系统发展遭遇瓶颈，Rust来拯救
2. Rust是WASI推广和普及和幕后推手
3. 基于Rust实现的新语言如雨后春笋般冒出，如Deno

## 3.3 Rust到哪里去

多个领域基础应用

Rust追求性能、安全和并发三连击语言

# 4 Rust语法再梳理

## 4.1 Rust语言版本

语义化版本

发行版本

Edition版次

## 4.2 Rust词法结构

编译器是Rustc

六大词法结构：

1. 关键字/保留字/若关键字
2.  标识符，字母或者下划线开头
3. 注释
4. 空白 
5. 词条

定义宏

```
#![allow(unused)]

macro_rules! calculate {
    // eval 1 + 2 $e是宏变量 expr 指类型
    (eval $e:expr) => {{
        {
            let val: usize = $e; //在这里用表达式把宏变量替换
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
```

6. 路径

路径有三种用法

```
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
```

## 4.3 Rust语法

### 4.3.1 表达式（上）

```
#！(allow(unused)) //允许代码中有未被使用的值
```

rust语法“骨架”

```
//属性，类似于 #！[...] 
```

```
// 分号; 返回的是一个单元值
```

```
// 花括号，{} //返回最后一个表达式的返回值
```

除了声明语句外，其他基本都是表达式

Rust 中一切皆表达式，而表达式又会产生值，值必定有类型，所以一切皆类型，另外操作符都是表达式

### 4.3.2 表达式（中）

**编译期计算**

编译期函数求值的简称

rust中使用1. 过程宏 + Build脚本（build.rs）2.类似于Cpp中constexpr的CTFE功能

rust编译期计算主要分为两大类：

常量函数 (const fn)/常量表达式与常量上下文

```
const X:T=...;
```

常量上下文包括：

1.常量初始化位置

2.静态数组的长度表达式，[T;N]

3.重复的长度表达式，类似：[0;10]

4.静态变量、枚举判别式的初始化位置

**常量函数/常量传播的区别**

1.常量传播是一种编译器优化

2.常量传播并不能改变程序任何行为，并且对开发者是隐藏的

3.编译期计算则是指编译执行时的代码，必须知道其结果才能继续执行

**常量函数与常量安全**

1.Rust中大部分表达式都可以用作常量表达式

2.并不是所有常量表达式都可以用在常量上下文，如一个数组长度依赖于磁盘中某个文件长度（引入常量函数解决）

常量函数还可以进行递归和内部嵌入

3.编译期求值必须得到一个确定性的结果

**常量泛型**

不同长度的数组，类型不同，由此引入常量泛型

```
#[feature(min_const_generics)]
use core::mem::MaybeUninit;
pub struct ArrayVec<T, const N: usize> {
    items: [MaybeUninit<T>; N],
    length: usize,
}
```

常量泛型是一种依赖类型

因为数组[T;N]的类型最终要依赖于N的具体值来确定

### 4.3.3 表达式（下）

Rust中表达式的分类：位置表达式和值表达式

位置上下文，有8个

**可变与不可变**

使用继承式或者可变绑定

**Rust中的类型**

1. 基本数据类型
2. 自定义复合类型
3. 容器类型

```
第一类：可变容器：UnsafeCell/Cell/RefCell
第二类：集合容器：Vec/HashMap
```

4. 泛型
5. 特定类型

**Rust中类型的行为**

Rust语言一切皆类型，字符串用双引号

1. trait抽象了不同类型的相同行为

2. trait是一种特设多态，比如整型的加法和字符串的加法
3. trait掌控类型行为逻辑，move和copy，copy trait的因素

Rust内置trait分类：

1. 所有权：Copy/Unpin/Drop
2. 并发：Sync/Send
3. 大小：Sized
4. 默认值：Default
5. 智能指针：Deref
6. 类型转换：From/Into/AsRef

### 4.3.4 语法面面观

#### 函数与闭包（上）

1. 常规函数

```
1. 函数拥有显式的类型签名
2. 函数分为三种：自由函数、关联函数 和 方法
3. 函数本身也是一种类型，但韩式的类型是一个零大小类型，元组和枚举也是
```

常规函数 /零大小类型的类型构造器

2. 函数指针

函数指针 /函数隐式转换为函数指针，在实际中尽量使用函数项类型，可以享受0大小类型优化

```
看某个类型占多大内存使用：println!("{:?}", std::mem::size_of_val(&c));
```

3. 闭包

闭包 / 函数无法捕获环境变量

```
// fn counter(i: i32) -> fn(i32) -> i32 {
//     fn inc(n: i32) -> i32 {
//         n + i // 函数不能捕获环境变量
//     }

//     inc
// }

// 使用闭包

fn counter(i: i32) -> impl FnMut(i32) -> i32 {
    move |n| n + i //转移所有权
}

fn main() {
    // let f = counter(2);

    // assert_eq!(3, f(1))
    let mut f = counter(2);
    assert_eq!(3, f(1))
}
```

闭包 /与函数指针互通

```
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
```

#### 函数与闭包（中）

闭包的实现原理：

```
1. 未捕捉到环境变量
2. 捕捉但修改环境变量
3. 捕捉但未修改环境变量
```

Fn/FnMut/FnOnce 这三者的关系是依次继承，正好对应于所有权语义三件套

闭包是编译器的语法糖

#### 函数与闭包（下）

逃逸闭包/非逃逸闭包

唯一不可变引用

闭包实现哪些trait

#### 模式匹配

Rust模式匹配是结构上匹配

```
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let (a, b) = (1, 2);

    let Point { x, y } = Point { x: 3, y: 4 };

    assert_eq!(1, a);
    assert_eq!(2, b);
    assert_eq!(3, x);
    assert_eq!(4, y);
}
```

支持模式匹配的位置

```
let 声明 let (a, b) = (1, 2); 不可辩驳模式
函数与闭包参数
match 表达式
if let 表达式
```

```
 //if let 
    let x = &Some(1);

    if let Some(y) = x {
        y;
    }
```

```
while let 表达式
for 表达式
```

#### 智能指针（上）

什么是智能指针？

涉及两个trait：Deref trait 和 Drop trait

如果一个类型实现了Deref trait，则它是一个智能指针指针

#### 智能指针（下）

智能在何处：1.自动解引用，提升开发体验； 2.自动化管理内存，安全无忧

标准库有哪些智能指针：Box<T>;Vec<T>和String；Rc<T>和Arc<T>;HashMap<K,V>

#### 字符和字符串（上）

**字符**

1. Unicode 标量值，其值对应于Rust中u32类型
2. 占4个字节

```
fn main() {
    let dao = '道';
    let dao_u32 = dao as u32;

    assert_eq!(36947, dao_u32)
}
```

**字符串**

1. UTF-8字节序列，“Vec<u8>”

2. str和String两大常用字符串类型，前者是切片类型

3. 其他字符串分类：

   CStr/CString

   OsStr/OsString

   path/PathBuf

```
//utf-8转字符串
    let dao = str::from_utf8(&[0xE9u8, 0x81u8, 0x93u8]).unwrap();
    assert_eq!("道", dao)
```

标准库的导读

```
1. 查看类型自身的介绍
2. 实现了哪些方法
3. 代码示例
```

#### 字符和字符串（下）

#### 集合容器（上）

[T]

Vec<T>

#### 集合容器（下）

HashMap和LinkedList

#### 迭代器（上）

**迭代器模式**

1. 设计模式中的一种行为模式
2. 常与集合使用，不暴露集合底层的情况下遍历集合元素
3. 将集合的遍历行为抽象为单独的迭代器对象

#### 迭代器（下）

IntoIter、Iter、IterMut

迭代器属于lazy，只有在消费的时候才会迭代

标准库文档和第三方库

#### 模块

模块是语法项的集合

用mod关键字定义，模块名小写，每个rust文件默认是一个模块

可以直接指定路径

#### Cargo包管理器（上）

让包直接声明各种依赖

1. 引入了Cargo.toml和Cargo.lock元数据文件
2. 提取并构建各种依赖包
3. 使用正确的参数来调用rustc或其他工具构建包
4. 引入了约定，使得使用Rust包更加容易

#### Cargo包管理器（下）

```
Cargo clean 	//清除cargo构建文件
cargo doc --open //生成文档，并打开，这种方法去看一个crate是最佳实践
Cargo fix //消除warning
cargo add  //安装只知道名字的包，自动添加为新的
cargo audit //检查依赖项
cargo clippy //静态分析代码，发现坏代码

其他的去看Cargo book	
```

#### 实际项目的组织结构（上）

#### 实际项目的组织结构（下）

根据业务组织代码

#### 定义自己的Crate（上）

# 5 Rust语言架构

掌握所有权语义

## 5.1 所有权

### 5.1.1 内存管理基础知识

### 5.1.2 安全管理之内存安全

### 5.1.3 生命周期和生命周期参数



