//定义类型并实现方法

struct User {
     name: &'static str,
 }
 
 impl User {
     fn name(&self) {
         println!("{:?}", self.name);
     }
 }
 
 use std::ops::Deref;
 
 // 定义类型并实现方法和trait
 struct MySmartPointer<T>(T);
 
 impl<T> MySmartPointer<T> {
     fn new(x: T) -> MySmartPointer<T> {
         MySmartPointer(x)
     }
 }
 
 impl<T> Deref for MySmartPointer<T> {
     type Target = T;
     fn deref(&self) -> &T {
         &self.0
     }
 }
 
 fn takes_str(s: &str) {
     println!("{:?}", s)
 }
 
 fn main() {
     let u = User { name: "Alex" };
 
     //泛型调用
     let y = MySmartPointer::new(u);
     y.name();
 
     let s = String::from("Hello world!");
     //可以看到这里的参数是String类型的引用
     let b = &s;
     //函数参数类型为字符串切片，&str，但是也能工作，因为String类型实现了Deref trait，能够解引用为&str
     takes_str(b);
 }