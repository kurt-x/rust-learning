/*!
 * # 内文档注释
 */

//! # 内文档注释
//!
//! 这个形式的文档注释属于包含此注释的项
//!
//! 例如，一个 crate 中的 //! 形式的文档注释用于注释 crate 本身，
//! 而不是跟在注释后面的元素

// 注释

// Rust 只支持单行注释，不支持多行注释

/// # 外文档注释
///
/// Rust 也支持文档格式的注释，可以使用 `///` 或 `//!` 开头
///
/// 这种注释会被 rustdoc 工具提取，生成 markdown 格式的文档
///
/// # Examples
///
/// ```rust
/// // 文档中的代码也会被 cargo test 测试
/// assert_eq!(1, 1);
/// ```
///
/// # Panics
///
/// 声明可能出现 panic 的情况
///
/// # Errors
///
/// 声明可能出现错误的情况
///
/// # Safety
///
/// 在包含 unsafe 的情况下，说明确保正常工作的条件
fn main()
{
    println!("Hello world!");
}

/**
 * # 外文档注释
 */
fn _fun() {}
