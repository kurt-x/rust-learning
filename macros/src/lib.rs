use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

// 自定义 derive
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream
{
    // 将 Rust 代码解析为语法树
    let ast: DeriveInput = dbg!(syn::parse(input).unwrap());

    let name = &ast.ident;
    let generated = quote!
    {
        impl HelloMacro for #name
        {
            fn hello_macro()
            {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };

    generated.into()
}

// 类属性宏
// attr 表示调用宏传入的属性，例如 GET, "/"
// item 表示宏附加到的项，例如 fn index() {}
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream
{
    dbg!(attr);
    dbg!(item);
    quote!().into()
}

// 类函数宏
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream
{
    dbg!(input);
    quote!().into()
}
