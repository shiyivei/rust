# 前置篇

## 1 内存：值放在堆上还是栈上

栈上数据：静态大小，静态生命周期

堆上数据，动态大小，动态生命周期

## 2 串讲：编程开发中的基本概念

### 2.1 数据：程序操作的对象

值、类型、指针和引用

类型和值息息相关：类型分为原生类型和组合类型

指针、引用、函数和闭包属于原生类型

组合类型：结构体标签联合

**指针**：是一个值，它持有存放数据的内存地址，通过解引用来访问它指向的内存地址，理论上可以解引用到任意数据类型。胖指针：还包含了字符串的长度和容量

**引用**：引用和指针类似，不同的是引用的解引用访问受限，只能解引用到它引用数据的类型，不能另做它用

### 2.2 代码：程序运行的主体

函数、方法、闭包、接口、虚表

接口是抽象层

### 2.3 运行方式：程序执行的效率

并发、并行、同步和异步

### 2.4 编程范式：提升代码质量

泛型编程

## 3 编写第一个Rust程序

## 4 get hands dirty,CLI 小工具

```
anyhow ="1" 错误处理
clap = "3.0.0-beta.4" //命令行解析工具，bata版本的crate会发生比较大的变化，甚至是类型和trait都不可再用
colored = "2" 命令行终端多色彩显示
jsonxf="1.1" 
mime = "0.3"
reqwest = {version = "0.11", features = ["json"]}
tokio = {version = "1", features =["full"]} 
```
