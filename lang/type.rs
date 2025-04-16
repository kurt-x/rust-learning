#![allow(dead_code)]
#![allow(unused_variables)]

// =>> 泛型、特征

use std:: {
    fmt:: { Debug, Display, },
    ops:: { Add, Deref },
};

// 创建一个类型别名，这种方式仅仅只是创建了一个“别名”
type StdVec<T> = std::vec::Vec<T>;

fn generic_type<T: Debug>(t: T) { dbg!(&t); }

#[derive(Debug)]
struct Point<T>
{
    x: T,
    y: T,
}

impl<T> Point<T>
{
    // 通用方法
    fn generic_method() { }
}

impl Point<f32>
{
    // 仅为 f32 实现的方法
    fn f32_only() { }
}

// 定义一个特征，类似其他语言的接口
pub trait Summary
{
    fn summarize(&self) -> String;

    // 也可以为特征方法添加方法体来作为默认实现
    // fn summarize(&self) -> String
    // {
    //     String::from("(Read more...)")
    // }
}

pub struct NewsArticle
{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// 如果要使用默认实现，使用空 impl 块
// impl Summary for NewsArticle { }

impl Summary for NewsArticle
{
    fn summarize(&self) -> String
    {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet
{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet
{
    fn summarize(&self) -> String { format!("{}: {}", self.username, self.content) }
}

// 指定一个参数是实现了 Summary 特征的类型
pub fn notify(item: &impl Summary) { println!("Breaking news! {}", item.summarize()); }
// 上面这行代码是下面这一行的简写，如果需要多个参数为同一类型，需要使用下面这行的形式
// pub fn notify<T: Summary>(item: &T) { println!("Breaking news! {}", item.summarize()); }

// 可以使用加号指定复合特征的类型
pub fn notify_with_add(item: &(impl Summary + Display)) { }
// pub fn notify_with_add<T: Summary + Display>(item: &T) { }

// 可以使用 where 指定类型
fn fun_with_where<T, U>(t: &T, u: &U)
where
    T: Display + Debug,
    U: Debug + Clone,
{ }

// 可以为实现了某个特征的类型实现特征，例如标准库中的一段代码
// impl<T: Display> ToString for T {...}

// 返回值指定实现某个特征的类型
fn returns_summarizable() -> impl Summary
{
    Tweet
    {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// =>> 特征对象
pub trait Draw
{
    fn draw(&self);
}

// Box<dyn Draw>> 是一个特征对象，代表任意实现了 Draw 的类型
// 泛型必须在编译时确定类型，特征对象可以在运行时使用多种类型
pub struct Screen
{
    pub components: StdVec<Box<dyn Draw>>,
}

pub struct Button
{
    height: usize,
    width: usize,
    label: String,
}

impl Draw for Button
{
    fn draw(&self)
    {
        println!("Button {}: height = {}, width = {}", self.label, self.height, self.width);
    }
}

fn main()
{
    generic_type(1);
    generic_type('c');
    generic_type(String::from("string"));

    let p = Point { x: 5, y: 10 };
    // p.f32_only(); // 报错，该方法只有 f32 类型可用
    dbg!(&p);
    let p = Point { x: 1.0, y: 4.0 };
    dbg!(&p);

    dbg!(Meter(1) + Meter(2));
    dbg!(Meter(1) + Kilometer(1));
}

// 这段代码会报错，因为只有特征或结构体其中一个或两者为当前箱的内容时才能实现特征
// impl Display for Vec<i32>
// {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { }
// }

// 这样是合法的
impl From<Button> for String
{
    fn from(item: Button) -> Self { item.label }
}

// 泛型生命周期
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str { if s1.len() > s2.len() { s1 } else { s2 } }

trait Foo<T = String> // 为泛型定义一个默认的类型
{
    type Inner; // 关联类型
}

#[derive(Debug)] struct Meter(i32);
#[derive(Debug)] struct Kilometer(i32);

impl Add for Meter
{
    type Output = Meter;
    fn add(self, other: Self) -> Self::Output { Self(self.0 + other.0) }
}

impl Add<Kilometer> for Meter
{
    type Output = Meter;
    fn add(self, other: Kilometer) -> Self::Output { Self(self.0 + other.0 * 1000) }
}

// NewType，用于包装类型
struct Vec(StdVec<String>);

// 如果想在自定义类型上使用内部类型的方法，可以实现 Deref
impl Deref for Vec
{
    type Target = StdVec<String>;

    fn deref(&self) -> &Self::Target { &self.0 }
}
