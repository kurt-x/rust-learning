// 数据

use std::mem::size_of;

// 常量等同于宏定义，会在内联时展开，没有固定内存
// 常量可以在任何地方声明，包括全局作用域，必须在声明时指定类型
// 常量的值必须是一个常量表达式，不能是函数调用的结果，或者任何只能在运行时计算的值
// const _CONST_VALUE: isize = generate_const_value(); // 这里会报错
const _CONST_VALUE: isize = 60 * 60 * 3;

// 静态变量拥有固定内存，属于数据
static _STATIC_VALUE: isize = 0;
static mut _STATIC_MUT_VALUE: isize = 0; // 可修改的静态变量，使用时需要 unsafe

fn main()
{
    let _var: i32;      // 声明变量，冒号后面声明类型，默认为不可变变量
    let mut _var: i32;  // 声明可变变量
    _var = 5;           // 赋值变量
    _var = 10;           // 重新赋值变量
    // 可以声明同名变量，新的变量会遮蔽（shadowing）上一个同名变量
    let _var = "hello";

    let _var = 10;      // 声明并赋值，会自动推断类型
    // _var = 20;       // 重新赋值会报错

    // 整型，整型字面量默认为 i32
    // i 开头为有符号整型，u 开头为无符号整型，后缀数字表示位数
    // isize 和 usize 位数取决于系统
    let _var = 10; // 整型字面量默认为 i32
    let _var = 10isize;
    let _var = 10usize;
    let _var = 10i8;
    let _var = 10i16;
    let _var = 10i32;
    let _var = 10i64;
    let _var = 10i128;
    let _var = 10u8;
    let _var = 10u16;
    let _var = 10u32;
    let _var = 10u64;
    let _var = 10u128;

    // 进制表示法
    let _dec = 98_222;
    let _hex = 0xff;
    let _oct = 0o77;
    let _bin = 0b1111_0000;
    let _byte = b'A'; // 仅限 u8 类型

    // 溢出，在 debug 模式下会报错，release 模式下会进行溢出处理
    let _of = 255u8.wrapping_add(1);
    let _checked = 255u8.checked_add(1);
    let _overflowing = 255u8.overflowing_add(1);
    let _saturating = 255u8.saturating_add(1);

    // 浮点型
    let _var = 1.0;     // 浮点型字面量默认为 f64
    let _var = 1.0f32;
    let _var = 1e-2;    // 科学计数法

    // 布尔型
    let _var = true;
    let _var = false;

    // 字符型，使用 Unicode 字符
    dbg!(&size_of::<char>()); // 4 bits
    let _var = 'a';
    let _var = 64;
    let _var = '😀';

    // 复合类型：元组（tuple）、数组（array）
    // 元组（tuple），可以包含多个不同类型的值，长度固定
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    dbg!(&tup.0,  &tup.1, &tup.2); // . 访问元组元素
    let (x, y, z) = tup;        // 模式匹配解构元组
    dbg!(&x, &y, &z);

    // 数组，所有元素类型必须相同，长度固定
    let arr: [i32;3] = [1, 2, 3];   // i32 表示元素类型，3 表示长度
    dbg!(&arr[0], &arr[1], &arr[2]);   // 下标访问数组元素，如果下标越界，debug 模式下报错，release 模式下会直接崩溃
    let _arr = [1; 3];               // 使用相同的值初始化数组，3 表示每个元素的初始值，5 表示长度

    // 切片
    let s = String::from("Hello, world!");
    let _sl = &s[0..5];     // 创建切片，从下标 0（包含）到 5（不包含）
    let _sl = &s[0..=5];    // 包含下标 5
    let _sl = &s[..];       // 从端点开始或结束可以省略下标值，这里表示获取整个切片

    // 任何集合类型都适用切片语法
    let v = vec![1, 2, 3, 4, 5];
    let _sl = &v[2..=3];

    // 结构体
    #[allow(dead_code)]
    #[derive(Debug)] // 使用默认方式实现 Debug 特征
    struct User
    {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // 方法
    impl User
    {
        // 方法的第一个参数始终是 &self，指代调用该方法的结构体实例
        // &self 是 self: &Self 的缩写，使用时用 self
        fn say_hi(&self) { println!("Hello, i'm {}", self.username); }

        // 没有构造函数语法，可以使用关联函数，比如 String::from
        fn with_name(username: &str) -> Self
        {
            Self
            {
                active: true,
                email: format!("{}@mail.com", username),
                username: String::from(username),
                sign_in_count: 1,
            }
        }
    }

    let username = String::from("someone");
    let mut user = User
    {
        active: true,
        username, // 上下文中有同名变量可以简写
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user.email = String::from("another@exampl.com");

    let new_user = User
    {
        username: String::from("another"),
        ..user // new_user 中剩余的属性使用 user 中的属性定义，改行必须放在最后
        // 结构更新语法会 move 原对象属性，原对象被用到的属性被消除
    };
    dbg!(&user.username);
    // dbg!(&user.email); // 报错
    dbg!(&new_user.email);

    let tom = User::with_name("tom");
    tom.say_hi();

    #[derive(Debug)]
    struct Color(u8, u8, u8); // 元组结构体

    let red = Color(255, 0, 0);
    let Color(r, g, b) = red; // 解构元组结构
    dbg!(&r, &g, &b);

    struct _Unit; // 定义一个单元结构体，相当于 Java 的 Object
    
    // 枚举
    #[allow(dead_code)]
    #[derive(Debug)]
    enum Message
    {
        // 枚举类似为结构体定义构造函数
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let _msg = Message::Move { x: 10, y: 20 };

    #[allow(unused_variables)]
    {
        // Option 是一个枚举类型，用于处理 None
        let none = None::<i32>;            // 声明 Option<i32> 类型的 None
        let none: Option<char> = None;     // 声明 Option<char> 类型的 None
        let opt = Some("Hello, world!");   // 声明 Option<&str> 类型变量
        let opt = Some('a');               // 声明 Option<char> 类型变量

        dbg!(&none.is_none());
        dbg!(&none.is_some());
        dbg!(&opt.is_none());
        dbg!(&opt.is_some());
    }
}
