#![allow(dead_code)]
#![allow(unused_variables)]
// 控制流程

fn main()
{
    // if 表达式
    let number = 3;

    if number < 5 { println!("number < 5"); }
    else if number == 5 { println!("number == 5"); }
    else { println!("number > 5"); }

    // Rust 中的 if 是一个表达式，可以使用在 let 语句中
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number = {number}");

    // loop 循环
    let mut counter = 0usize;

    let n = 'lebal:  loop // 使用 'lebal: 可以给 loop 循环命名
    {
        counter += 1;

        if counter == 3 { continue; } // 使用 continue 跳过本次循环

        println!("loop {counter}");

        if counter == 5 { break counter * 2; } // 使用 break 退出循环，循环可以带返回值

        if counter == 4 { break 'lebal 1; } // 使用 break 'lebal; 或 continue 'lebal; 跳出或继续指定的循环

        // break 仅是退出当前循环，使用 return 退出整个函数
    };
    println!("n = {n}");

    // while 循环
    let mut number = 3;

    while number != 0
    {
        println!("while {number}");
        number -= 1;
    }

    // for 循环
    let arr = [10, 20, 30, 40, 50];

    for e in arr { println!("for {e}"); }

    for (i, e) in arr.iter().enumerate() { println!("Element {i}: {e}"); }

    for n in (1..4).rev() { println!("{n}!"); }

    // match 表达式
    let m = value_in_cents(Coin::Quarter(UsState::Alaska));
    dbg!(&m);

    let dice_roll = 9;
    let cond = false;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        1 | 2 => nothing(), // 或
        10..20 => nothing(), // 范围匹配
        20..=30 => nothing(), // 包含 30
        31..40 if dice_roll % 2 == 0 => nothing(), // 匹配守卫
        other => move_player(other), // 通过声明变量的形式作为默认分支，类似其他语言中的 default 分支
        // _ => (), // 也可以使用 _ 匿名变量表示只需要使用默认分支，但不需要声明变量
    }

    let var = 'c';
    match var
    {
        'a'..='z' => nothing(),
        _ => println!("no matched"),
    }

    fn nothing() {}
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}

    // if let
    // 可以使用 if let 语法简化 match
    let coin = Coin::Quarter(UsState::Alabama);
    if let Coin::Quarter(state) = coin { dbg!(&state); }
    else { } // 类似于 _ => ()

    // if 块可省略
    let x: Option<i32> = None;
    let Some(v) = x else // 没有 if，如果匹配不上（x 为 None），只能返回，否则编译器会报错
    {
        println!("None");
        // ..do sth
        return;
    };

    #[allow(irrefutable_let_patterns)]
    let a = 1 else { return; }; // 可以编译，但是没有意义，编译器会发出警告

    struct Point<T> { x: T, y: T }

    let p = Point { x: 5, y: 10 };
    let Point { x: a, y: b } = p; // 模式匹配解构结构体
    let Point { x, y } = p; // 同创建一样，解构也可以简写
    let Point { x, .. } = p; // 省略部分
    let Point { x: 10..20, y: z @ 10..20  } = p else { return; }; // 指定范围匹配
}

impl UsState
{
    fn existed_in(&self, year: u16) -> bool
    {
        match self
        {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String>
{
    // 可以使用 let-else 简化 if let - else
    let Coin::Quarter(state) = coin else { return None; };

    if state.existed_in(1900) { Some(format!("{state:?} is pretty old, for America!")) }
    else { Some(format!("{state:?} is relatively new.")) }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => 
        {
            println!("State quarter from {state:?}!");
            25
        }
    }
}
