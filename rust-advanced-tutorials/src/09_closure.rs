//能为函数返回并且不在函数调用过程中被销毁的闭包，逃逸闭包

// #![feature(unboxed_closures, fn_traits)]

fn c_mut() -> impl FnMut(i32) -> [i32; 3] {
     let mut arr = [0, 1, 2];
     move |i| {
         arr[0] = i;
         arr
     }
 }
 
 // 非逃逸闭包
 
 // fn c_mut2() -> impl for<'a> FnMut(&'a str) -> String {
 //     let mut s = "hello".to_string() ; //动态可增长的字符串，没有实现copy trait
 
 //     //并不能安全返回 闭包
 //     // move |i| {
 //     //     s += i;
 //     //     s
 //     // }
 
 // }
 
 fn main() {
     let i = 42;
 
     let mut arr_closure = c_mut();
 
     println!("{:?}", arr_closure(i));
 }
 