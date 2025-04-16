// 集成测试
// tests 目录下的模块不需要加 #[cfg(test)]
// 如果项目中没有 lib.rs（项目不是一个库），就不可以创建集成测试

use module::add;

mod common;

#[test]
fn test_add() { assert_eq!(add(2, 2), 4); }
