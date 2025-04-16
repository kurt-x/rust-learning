// 所有权
// 所有权是 Rust 特有的特性，用于保障内存安全

fn main()
{
    // 具有固定大小的值，会存储在栈中，不会有所有权问题
    let x = 10;
    let y = x;

    dbg!(&x, &y);

    let x = String::from("hello");
    let y = x; // 此时所有权发生改变，x 指向的对象被交给了 y，x 无法继续使用
    // 这个行为在其他语言中称为浅拷贝，不会有任何问题
    // 在 Rust 中被称为移动（move）

    // println!("{x} World!"); // 报错
    println!("{y} World!");
    
    let y = String::from("hi"); // y 被赋予了新值，原来的字符串 hello 会立刻调用 drop 被回收
    println!("{y} World!");
    
    let x = "test";
    #[allow(noop_method_call)]
    let y = x.clone(); // 深拷贝可以避免移动报错
    
    println!("{x} sth");
    println!("{y} sth");

    // 如果一个类型实现了 Copy 特征，也会有和 clone 方法一样的效果
    // Copy 和 Drop 互斥！！
    // 所有整数类型、所有浮点数类型以及布尔类型、字符类型都实现了 Copy 特征
    // 如果元组中的所有类型都实现了 Copy 特征，那么才可以认为该元组具备 Copy 特征

    // 变量传参的行为也与赋值相同
    let s = String::from("process");
    process_string(s);
    // println!("after processed: {s}"); // 报错

    // 引用变量可以防止所有权问题
    let s = String::from("reference");
    reference_string(&s);
    println!("after referenced: {s}");
}

fn process_string(s: String)
{
    // s 进入函数
    println!("processed string: {s}");
    // 退出函数，变量 s 自动 drop
}

fn reference_string(s: &String)
{
    println!("referenced string: {s}"); // s 会被自动解引用
}
