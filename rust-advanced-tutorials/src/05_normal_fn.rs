//自由函数
fn sum(a: i32, b: i32) -> i32 {
     a + b
 }
 
 struct A(i32, i32);
 
 impl A {
     //关联函数
     fn sum(a: i32, b: i32) -> i32 {
         a + b
     }
 
     //方法
     fn math(&self) -> i32 {
         Self::sum(self.0, self.1)
     }
 }
 
 //类型构造器
 enum Color {
     R(i16),
     G(i16),
     B(i16),
 }
 
 //等价于如下；Color::R是类型构造器
 // fn Color::R(_1:i16) -> Color {/*..*/}
 // fn Color::G(_1:i16) -> Color {/*..*/}
 // fn Color::B(_1:i16) -> Color {/*..*/}
 
 fn main() {
     assert_eq!(3, sum(1, 2));
 
     let a = A(1, 2);
 
     // 关联函数调用，使用路径分割符
     assert_eq!(3, A::sum(1, 2));
 
     // 方法使用点调用
     assert_eq!(3, a.math());
 
     let add = A::sum; //Fn item整个类型，表现出了函数本身也是一个类型
 
     let add_math = A::math;
 
     assert_eq!(add(1, 2), A::sum(1, 2));
     assert_eq!(add_math(&a), a.math());
 
     //零大小类型
 
     println!("{:?}", std::mem::size_of_val(&Color::R));
     println!("{:?}", std::mem::size_of_val(&Color::G));
     println!("{:?}", std::mem::size_of_val(&Color::B))
 }
 