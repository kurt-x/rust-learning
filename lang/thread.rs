#![allow(dead_code)]

// 并发编程

struct Foo;

// Send 特征表示可以在线程间传递所有权，Rust 类型基本都实现了 Send
// 完全由 Send 类型组成的类型会自动被标记为 Send

// Sync 特征表示一个类型的引用是线程安全的
// 完全由 Sync 类型组成的类型会自动被标记为 Sync

// 通常不需要手动实现
unsafe impl Send for Foo { }
unsafe impl Sync for Foo { }
// Rc 不是 Send 或 Sync，多线程下使用 Arc

use std::
{
    thread,
    time::Duration,
    // mpsc: multiple producer, single consumer 多生产者，单消费者
    // mpmc: multiple producer, multiple consumer 多生产者，多消费者
    sync:: { mpsc, Mutex, Arc },
};

const DURATION: Duration = Duration::from_millis(1);
// const DURATION: Duration = Duration::from_millis(500);

fn main()
{
    // 创建一个系统级线程
    let trd = thread::spawn(|| {
        for i in 1..11
        {
            println!("子线程 {i}");
            thread::sleep(DURATION);
        }
    });

    // trd.join().unwrap(); // 阻塞当前线程直到 trd 线程执行完成

    for i in 1..4
    {
        println!("主线程 {i}");
        thread::sleep(DURATION);
    }

    trd.join().unwrap();
    println!("=> Done");

    // tx(Transmit) 发送
    // rx(Receiver) 接收
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(String::from("Hello")).unwrap_or_else(
            |e| println!("无法发送，接收端可能已被丢弃：{e}")
        );
    });

    // recv() 会阻塞直到收到可用值
    // try_recv() 不会阻塞，立即返回一个 Result<T, E>
    let receive = rx.recv();
    match receive
    {
        Ok(m) => println!("接收成功：{m}"),
        Err(e) => println!("接收失败：{e}"),
    }

    // loop_send();
    // multi_send();

    // Mutex(Mutual Exclusion) 互斥，同一时间只允许一个线程访问其中数据
    // Arc(Atomically Reference Counted) 原子引用计数
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10
    {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles { handle.join().unwrap(); }

    println!("Result: {}", *counter.lock().unwrap());
}

fn loop_send()
{
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let data = vec![1, 2, 3, 4, 5];
        for i in data
        {
            tx.send(i).unwrap();
            thread::sleep(DURATION);
        }
    });

    for received in rx { println!("Got: {received}"); }
}

fn multi_send()
{
    // --snip--

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec!
        [
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals
        {
            tx1.send(val).unwrap();
            thread::sleep(DURATION);
        }
    });

    thread::spawn(move || {
        let vals = vec!
        [
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals
        {
            tx.send(val).unwrap();
            thread::sleep(DURATION);
        }
    });

    for received in rx { println!("Got: {received}"); }
}
