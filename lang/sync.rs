#![allow(dead_code)]

use std::time::Duration;
use trpl::{ Either, Html, StreamExt };

fn main()
{
    // get_page_title();
    // counting_in_async();
    stream();
}

fn stream()
{
    trpl::run(async {
        let mut stream = trpl::stream_from_iter(vec![1, 2, 3].into_iter());
        while let Some(value) = stream.next().await { println!("{:?}", value); }
    });
}

const  DURATION: Duration = Duration::from_millis(500);

fn counting_in_async()
{
    trpl::run(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10
            {
                println!("子任务 {i}");
                trpl::sleep(DURATION).await;
            }
        });

        for i in 1..10
        {
            println!("主任务 {i}");
            trpl::sleep(DURATION).await;
        }

        handle.await.unwrap(); // 使用 await 可以类似线程 join 功能
    });
}

fn get_page_title()
{
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        // Future 是惰性的，在调用 await 之前不会执行
        let title1 = page_title(&args[1]);
        let title2 = page_title(&args[2]);

        // 同时调用，获取最先返回的值
        let (url, maybe_title) = match trpl::race(title1, title2).await
        {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{url} 最先返回");
        match maybe_title
        {
            Some(title) => println!("{url} 的标题为 {title}"),
            None => println!("{url} 不包含标题"),
        }
    })
}

async fn page_title(url: &str) -> (&str, Option<String>)
{
    let content = trpl::get(url).await.text().await;
    let title = Html::parse(&content)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}

// page_title() 会被编译成下面这种形式（函数名是相同的，这里为了区分）
// Future 返回值必须明确指出受生命周期约束，但不需要命名，编译器可以推断出
fn async_page_title(url: &str) -> impl Future<Output=Option<String>> + '_
{
    async move {
        let content = trpl::get(url).await.text().await;
        Html::parse(&content)
            .select_first("title")
            .map(|title| title.inner_html())
    }
}
