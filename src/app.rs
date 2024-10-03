use std::io;
use crate::util;
use colored::Colorize;

pub struct App;

impl App {
    pub fn run(&self) {
        println!("欢迎使用FF14鼠标指向宏生成器");
        println!("提示：按Ctrl+C退出程序");
        loop {
            println!("请输入技能名：");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect(&"读取输入失败".red());
            let trimmed = input.trim();
            if trimmed.is_empty() {
                eprintln!("{}", "输入不能为空，请重新输入".red());
                continue;
            }

            let mac = format!("/micon {trimmed}
/merror off
/ac {trimmed} <mo>
/ac {trimmed}");
            println!("生成的指向宏：");
            println!("{}", mac.blue());

            util::write_to_clipboard(&mac).expect(&"写入剪贴板失败".red());
            println!("{}", "已复制到剪贴板".green());
            println!();
        }
    }
}