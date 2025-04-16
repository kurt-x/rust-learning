#![allow(unused_variables)]
#![allow(dead_code)]

// 错误处理
use std::fs:: { self, File };
use std::io:: { self, ErrorKind, Read };

fn main()
{
    // panic!("has error"); // panic 宏会强制终止程序
    // 一些方法会返回 Result<T, E> 枚举类，T 是正常返回值，E 是异常

    // let file = read_file();
    let file = read_file_with_unwrap();
    dbg!(&file);

    // let content = read_file_without_handle();
    // dbg!(&content);
}

fn read_file() -> File
{
    match File::open("temp/hello.txt")
    {
        Ok(file) => file, // 如果文件打开成功返回 Result 的 Ok 
        Err(error) => match error.kind() // 若失败返回 Err
        {
            ErrorKind::NotFound => match File::create("temp/hello.txt")
            {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {}", e),
            },
            other_error => { panic!("Problem opening the file: {}", other_error); }
        },
    }
}

fn read_file_with_unwrap() -> File
{
    // 如果返回值是 Ok，unwrap 会返回 Ok 内的值，否则调用 panic!
    // File::open("temp/hello.txt").unwrap()
    // expect 等同于可以自定义错误信息的 unwrap
    File::open("temp/hello.txt").expect("自定义错误消息")
}

fn read_file_without_handle() -> Result<String, io::Error>
{
    let mut content = String::new();
    File::open("temp/hello.txt")?.read_to_string(&mut content)?; // 若不想处理，可以使用 ? 抛出
    Ok(content)
    // 上面的代码等同于：
    // let file_result = File::open("temp/hello.txt");
    // let mut file = match file_result { Ok(f) => f, Err(e) => return Err(e) };
    // match file.read_to_string(&mut content) { Ok(_) => Ok(content), Err(e) => Err(e) }
}

fn read_file_with_fs() -> Result<String, io::Error> { fs::read_to_string("temp/hello.txt") }
