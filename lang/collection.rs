#![allow(unused_variables)]

fn main()
{
    println!("=== 向量 ===");
    let vec: Vec<i32> = Vec::new(); // 创建一个向量
    dbg!(&vec);
    let vec = vec![1, 2, 3]; // 使用宏创建一个向量
    dbg!(&vec);

    // 获取向量中的值
    let e = &vec[2]; // 返回数字
    dbg!(&e);
    let e = vec.get(2); // 返回 Option<&i32>
    dbg!(&e);

    println!("\n=== 字符串 ===");
    // 字符串内部使用字符类型的向量
    let str = String::new(); // 创建一个空字符串
    let str = String::from("hello"); // 以一个字符串切片创建字符串
    for c in str.chars() { print!("{c}"); } // 由于 utf-8 的底层特性，遍历字符串需要使用 chars 方法
    println!();

    println!("\n=== HashMap ===");
    use std::collections::HashMap; // HashMap 需要引入
    // HashMap
    let mut map = HashMap::new();
    // insert 会获取所有 key 和 value 的所有权
    map.insert(String::from("Red"), 10);
    map.insert(String::from("Blue"), 50);

    dbg!(map.get(&String::from("Blue")));

    for (k, v) in map { println!("key: {k}, value: {v}"); }
}
