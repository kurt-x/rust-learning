// 智能指针

use std::cell::RefCell;
use std::rc::Rc;

fn main()
{
    // Box
    // 将数据存在堆上，用于解决递归类型，循环引用或数据过大等情况
    // Box 只提供间接存储和对分配功能
    let var = Box::new(5);
    dbg!(&var);

    // 解引用
    let x = 5;
    let y = &x;

    // assert_eq!(x, y); // 报错，x 是数值，y 是指针，无法进行比较
    assert_eq!(x, *y);

    let y = Box::new(x);
    assert_eq!(x, *y); // Box 实现了 Deref，所以 Box 的解引用与普通指针的解引用效果相同

    // Rc，引用计数（reference counting）
    // 会记录被引用数量，计数为 0 时表示没有引用，可以被清除
    let data = Rc::new(5);
    let x = Rc::clone(&data);
    let y = Rc::clone(&data);
    let z = Rc::clone(&data);
    let _w = Rc::downgrade(&data); // 创建弱引用
    dbg!(Rc::strong_count(&data));
    dbg!(Rc::weak_count(&data));
    dbg!(x, y, z);
    dbg!(Rc::strong_count(&data));
    dbg!(Rc::weak_count(&data));

    // RefCell，该指针内部使用 unsafe 代码实现可变性
    // RefCell 会以运行时 panic 的方式保证安全性，而不是编译检查的方式
    let data = RefCell::new(5);
    let x = data.borrow_mut();
    // let y = data.borrow_mut(); // 报错，创建了多个可变引用
    dbg!(x);
}
