fn main()
{
    cc::Build::new()
        .file("hello.c")
        .compiler("clang") // 用 clang
        .compile("hello");
}
