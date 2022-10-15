use std::fs; //fs 包用来读取或者文件操作

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.rust-lang.org";

    let output = "rust.md";

    println!("Fetch url is {:?}", url);

    let body = reqwest::blocking::get(url)?.text()?;
    println!("Converting html to markdown... ");

    let md = html2md::parse_html(&body);
    fs::write(output, md.as_bytes())?;

    //unwrap 这种方式事实上阻断了错误的传播
    //如果想传播错误，请使用？替换unwrap,此时会返回Result<T,E>
    //然后最底下用Ok(())
    //返回值使用Box包裹

    println!("Converted markdown has been saved in {}.", output);

    Ok(())
}
