#![allow(dead_code)]

// 单元测试
// cargo test
// 测试默认使用多线程 --test-threads=1 可以设置为单线程
// --show-output 可以显示 println! 的输出
// cargo test [part_name] 会模糊匹配对所有测试模块和方法模糊匹配 [part_name]

pub fn add(l: usize, r: usize) -> usize { l + r }

#[test]
fn inner_test_add() { assert_eq!(add(2, 2), 4); }

#[cfg(test)]
mod test_module
{
    use crate::add;

    #[test]
    #[should_panic(expected = "失败")] // 声明该测试必须发出 panic，expected 表示失败原因必须包含这段字符串
    fn test_fn()
    {
        let result = add(1, 1);
        assert!(result == 2); // 断言
        assert_ne!(result, 1); // 不等于
        assert_eq!(result, 2); // 等于
        debug_assert!(result != 0); // 非 debug 模式不会编译该行代码

        assert!(result != 2, "断言失败");
    }

    #[test]
    #[ignore] // 忽略该测试
    // -- --ignored 参数只测试被忽略的测试
    // -- --include-ignored 参数在测试中包含被忽略的测试
    fn ignored_test() { }
}
