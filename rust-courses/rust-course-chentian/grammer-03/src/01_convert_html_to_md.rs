/// fs 是一个关于对文件操作的包，可以用来将字符串写到文件
use std::fs;

fn main() {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";

    //从路径请求内容
    println!("Fetchingb url:{}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    //解析请求到的内容
    println!("Convert html to markdown...");
    let md = html2md::parse_html(&body);

    //保存到本地
    fs::write(output, md.as_bytes()).unwrap();

    println!("Converted markdown has been saved in {}", output);
}