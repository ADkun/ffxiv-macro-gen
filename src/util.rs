use clipboard::{ClipboardContext, ClipboardProvider};
use std::path::Path;
use std::error::Error;
use std::env;
use std::path::PathBuf;

pub fn write_to_clipboard(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(text.to_owned())?;
    Ok(())
}

pub fn read_file_content(path: impl AsRef<Path>) -> Result<String, Box<dyn Error>> {
    let content = std::fs::read_to_string(path)?;
    Ok(content)
}

pub fn pause() {
    println!("按回车继续...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}

pub fn get_exe_dir() -> PathBuf {
    let exe_path = env::current_exe().expect("获取当前可执行文件路径失败");
    exe_path.parent().expect("获取父目录失败").to_path_buf()
}