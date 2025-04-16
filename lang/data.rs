// 数据

// 常量可以在任何地方声明，包括全局作用域
// 常量的值必须是一个常量表达式，不能是函数调用的结果，或者任何只能在运行时计算的值
// const THREE_HOURS_IN_SECONDS: u32 = get_three_hours_in_seconds(); // 这里会报错
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main()
{
    println!("=== 变量声明 ===");

    let var = 10; // 声明变量，Rust 默认变量不可变，且会自动推断类型

    dbg!(&var);
    // var = 20; // 重新赋值会报错

    let mut var2 = 10; // 使用 mut 关键字声明可变变量

    // var2 = 20; // 在变量被使用之前不可以被重新赋值，这里会报错

    dbg!(&var2);
    var2 = 20; // 可以重新赋值
    dbg!(&var2);

    // 使用 const 声明常量，常量必须指定类型
    // 且常量不可以搭配 mut 使用，始终是不可变的
    const VAR3: i32 = 10;
    dbg!(&VAR3);
    dbg!(&THREE_HOURS_IN_SECONDS);

    // 遮蔽（shadowing）：可以在变量名相同的情况下重新声明变量
    let var = var + 10;
    dbg!(&var);
    // 遮蔽后的变量可以改变类型
    let var = "hello";
    dbg!(&var);

    println!("\n=== 数据类型 ===");

    // Rust 中的类型分为两类：标量（scalar）和复合（compound）
    // 标量类型：整型、浮点型、布尔型、字符型

    // 整型：i8、i16、i32、i64、i128、u8、u16、u32、u64、u128、isize、usize
    // i 表示有符号整数，u 表示无符号整数，后缀数字表示位数
    // isize 和 usize 类型依赖运行程序的计算机架构，64 位架构上是 64 位，32 位架构上是 32 位

    // 整型默认是 i32 类型，可以使用后缀指定类型
    let int: i32 = 10;
    dbg!(&int);
    let int = 10i64;
    dbg!(&int);

    // Rust 字面量同样可以使用进制表示
    let decimal = 98_222; // 十进制
    let hex = 0xff; // 十六进制
    let octal = 0o77; // 八进制
    let binary = 0b1111_0000; // 二进制
    let byte = b'A'; // 字节（仅限 u8 类型）
    dbg!(&(decimal, hex, octal, binary, byte));

    // 溢出：在 debug 模式下会报错，release 模式下会进行溢出处理
    // let overflow = 255u8 + 1; // 这里会报错，除非使用溢出处理或指定了 --release 参数
    let overflow = 255u8.wrapping_add(1);
    dbg!(&overflow);
    let checked_overflow = 255u8.checked_add(1);
    dbg!(&checked_overflow);
    let overflowing_overflow = 255u8.overflowing_add(1);
    dbg!(&overflowing_overflow);
    let saturating_overflow = 255u8.saturating_add(1);
    dbg!(&saturating_overflow);

    // 浮点型：f32、f64
    let f = 2.0; // 默认是 f64 类型
    dbg!(&f);
    let f: f32 = 2.0; // 可以使用后缀指定类型
    dbg!(&f);
    let f = 2.0f32;
    dbg!(&f);
    let f = 2e-2; // 支持科学计数法
    dbg!(&f);

    // 布尔型：bool
    let b = true;
    dbg!(&b);
    // 字符型：char
    let c = 'a';
    dbg!(&c);
    let c = 64;
    dbg!(&c);
    let c = '😀'; // Rust 中的字符是 Unicode 字符
    dbg!(&c);

    // 复合类型：元组（tuple）、数组（array）
    // 元组：可以包含多个不同类型的值，长度固定
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tuple = ({}, {}, {})", tup.0, tup.1, tup.2); // 使用 . 访问元组的元素
    let (x, y, z) = tup; // 可以使用模式匹配解构元组
    println!("x = {x}, y = {y}, z = {z}");

    // 数组：所有元素类型必须相同，长度固定
    let arr = [1, 2, 3, 4, 5];
    println!("array = {}, {}, {}, {}, {}", arr[0], arr[1], arr[2], arr[3], arr[4]); // 使用下标访问数组元素
    // 通过下标访问数组元素时，如果下标越界，Rust 会在 debug 模式下报错，release 模式下会直接崩溃
    let arr: [i32; 5] = [1, 2, 3, 4, 5]; // 指定类型和长度，i32 表示元素类型，5 表示长度
    println!("array = {}, {}, {}, {}, {}", arr[0], arr[1], arr[2], arr[3], arr[4]);
    let arr = [3; 5]; // 使用相同的值初始化数组，3 表示每个元素的初始值，5 表示长度
    println!("array = {}, {}, {}, {}, {}", arr[0], arr[1], arr[2], arr[3], arr[4]);

    println!("\n=== 切片 ===");
    let s = String::from("hello world");
    let slice = &s[0..5]; // 创建 s 的切片，从位置 0（包含）到位置 5（不包含），如果切片从 0 开始，0 可以省略 &s[..5]
    dbg!(&slice);
    let slice = &s[6..11]; // 可以省略 11 表示从 6 一直到结尾的位置
    dbg!(&slice);
    let slice = &s[..]; // 获取整个字符串的切片
    dbg!(&slice);

    // 任何集合类型都适用切片语法
}
