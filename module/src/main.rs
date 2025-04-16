#![allow(unused_imports)]
#![allow(dead_code)]

// 声明模块

// 声明一个内联模块
mod inner_module
{
    pub use deep_module::deep_function; // 将私有模块的内容重新导出

    // 声明一个子模块
    pub mod sub_module
    {
        use self::super::super::module; // self 表示自身，super 表示父模块
    }

    mod deep_module
    {
        pub fn deep_function() { }
    }

    pub mod nested_module
    {
        pub fn fun1() { }
        pub fn fun2() { }
    }
}
// 声明一个外部模块
mod module;
// 编译器会查找 src/module.rs 或 src/module/mod.rs

// 声明一个公有模块，外部可访问
pub mod public_module { }

fn main() {
    use module; // 使用相对路径引用一个模块
    // 使用绝对路径引用一个模块
    use crate::inner_module::sub_module; // crate:: 表示当前包

    use crate::public_module as pub_module; // 使用 as 关键字指定新名称

    use inner_module::deep_function; // 使用重新导出的内容

    use inner_module::nested_module:: { self, fun1, fun2 }; // 嵌套引入，self 表示自身

    use module::*; // 全部引入
}
