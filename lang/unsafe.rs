// 不安全代码

// 静态变量
// 全局使用唯一地址
static _HELLO_WORLD: &str = "Hello World!";
// 可以声明一个可变静态变量
static mut _COUNTER: u32 = 0;
// 访问或修改可变静态变量是不安全的，需要使用 unsafe

fn main()
{
    // 原始指针
    // 可以同时拥有一个变量的可变引用和不可变引用
    // 不能保证指向有效内存
    // 允许为空
    // 不会有任何自动清理

    let mut num = 5;

    let rp1: *const i32 = &raw const num; // 不可变的原始指针
    let rp2: *mut i32 = &raw mut num; // 可变的原始指针

    let address = 0x012345usize;
    let _rp = address as *const i32; // 将一个数字解释为一个原始指针
    // 解引用原始指针只能放在 unsafe 块中
    unsafe
        {
            println!("rp1: {}", *rp1);
            println!("rp2: {}", *rp2);
            dangerous(); // 调用不安全函数
        }

    say_hi();
}

unsafe fn dangerous() {}

// 调用外部函数
unsafe extern "C"
{
    fn _abs(input: i32) -> i32;

    // 使用 safe 关键字可以指示一个外部函数是安全的，调用这个函数时就不需要 unsafe 块
    safe fn say_hi();
}

// 使用 extern 创建一个可供外部函数调用的接口
#[unsafe(no_mangle)] // 指示编译器不要修改函数名称
pub extern "C" fn ext_api() {}

unsafe trait _Foo { }
unsafe impl _Foo for i32 { }

// 与 C 中的 union 相同
union _Value
{
    int: i32,
    float: f32,
    bytes: [u8; 4],
}
