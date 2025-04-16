#![allow(unused_variables)]
#![allow(dead_code)]

// 声明式宏

//! # 宏片段说明符：
//! | 说明符 | 匹配内容 | 示例 |
//! |---------------|----------------------------------|------------------------------|
//! | `ident`       | 标识符（如变量名、函数名）         | `foo`, `bar`                 |
//! | `expr`        | 表达式                           | `1 + 2`, `some_func()`       |
//! | `ty`          | 类型                             | `u32`, `Vec<T>`              |
//! | `pat`         | 模式                             | `Some(x)`, `Ok(_)`           |
//! | `path`        | 路径                             | `std::mem::replace`          |
//! | `tt`          | 标记树（Token Tree），几乎匹配任何内容 | `1 + (2 * 3)`, `{ let x = 5; }` |
//! | `block`       | 代码块                           | `{ println!("Hi"); }`        |
//! | `stmt`        | 语句                             | `let x = 3;`                 |
//! | `item`        | 条目（如函数、结构体、模块等）      | `fn foo() {}`                |
//! | `meta`        | 元信息（如属性）                   | `#[derive(Debug)]`           |
//! | `literal`     | 字面量                         | `"hello"`, `42`              |
//! | `lifetime`    | 生命周期标注                   | `'a`, `'static`              |
//! | `vis`         | 可见性修饰符                      | `pub`, `pub(crate)`          |

use macros::{ route, sql };

#[macro_export] // 导出宏，使外部可用
macro_rules! status_enum
{
    // ident    标识符（变量名/函数名等）
    // expr     表达式
    // ty       类型
    // pat      模式

    ($($name:ident: $code:literal, $msg:literal)*) =>
    {
        impl Status
        {
            $(const $name: Status = Status { code: $code, message: $msg };)*
        }
    };
}

struct Status
{
    code: u16,
    message: &'static str,
}

status_enum!
{
    SUCCESS:    200, "OK"
    ERROR:      500, "Internal Server Error"
    NOT_FOUND:  404, "Not Found"
}

// 过程宏，分为【自定义 derive】、【类属性宏】和【类函数宏】（类表示类似 like）

// 自定义 derive
trait HelloMacro
{
    fn hello_macro();
}

#[route(Get, "/index")]
#[derive(HelloMacro)]
struct Pancakes {}

fn main()
{
    let status = Status::NOT_FOUND;
    println!("Code: {}, Message: {}", status.code, status.message);

    sql!(SELECT * FROM foo WHERE id=1);
}
