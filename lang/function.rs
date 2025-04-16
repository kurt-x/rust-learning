#![allow(unused_variables)]

// 函数

fn main()
{
    println!("Hello world!");

    another_function(10);

    // Rust 中的块可以有返回值
    let x =
    {
        let y = 10;
        y + 1 // 注意这里没有分号，表示这是一个表达式，而非语句
    };
    dbg!(x);

    let x = add(1, 1);
    dbg!(x);

    println!("\n=== 闭包 ===");
    // 闭包
    let closure = |num: isize| -> isize { num };
    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // v3 和 v4 版本必须被使用，编译器需要通过使用才能推断出函数类型
    let add_one_v3 = |x|             { x + 1 };
    let add_one_v4 = |x|               x + 1  ;

    let var = add_one_v3(1usize);
    dbg!(var);
    let var = add_one_v4(1i8);
    dbg!(var);

    use std::thread;

    let list = vec![1, 2, 3];
    println!("定义闭包之前：{list:?}");

    // move 关键字强制获取捕获变量的所有权
    thread::spawn(move || println!("线程内：{list:?}"))
        .join()
        .unwrap();
}

fn another_function(x: i32)
{
    println!("Another function");
    dbg!(x);
}

fn add(x: i32, y: i32) -> i32
{
    x + y
    // 也可以使用 return 关键字返回值
    // return x + y;
}
