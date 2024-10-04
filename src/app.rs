use std::io;
use crate::util;
use std::path::Path;
use colored::Colorize;
use std::fs;
use std::path::PathBuf;

const MACRO_FOLDER: &str = "macros";

pub struct App {
    macro_dir: PathBuf,
}

impl App {
    pub fn new() -> Self {
        let mut macro_dir = util::get_exe_dir();
        macro_dir.push(MACRO_FOLDER);
        Self { macro_dir, }
    }

    pub fn run(&self) {
        self.check_folder();

        println!("欢迎使用FF14宏生成器");
        println!("提示：按Ctrl+C退出程序");
        println!();

        let names = self.get_macro_txt_names();
        loop {
            println!("可使用的宏模板：");
            for (index, name) in names.iter().enumerate() {
                println!("{}: {name}", index + 1);
            }
            println!("请输入数字以选择模板：");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect(&"读取输入失败".red());
            let input = input.trim();

            let index = match input {
                "" => {
                    eprintln!("{}", "输入不能为空，请重新输入".red());
                    continue;
                }
                s => {
                    let index = match s.parse::<usize>() {
                        Ok(index) => index,
                        Err(_) => {
                            eprintln!("{}", "输入必须为正整数，请重新输入".red());
                            continue;
                        }
                    };
                    if index > names.len() || index == 0 {
                        eprintln!("输入超出范围 [{} - {}]，请重新输入", 1, names.len());
                        continue;
                    }
                    index - 1
                }
            };

            let name = &names[index];
            let path = Path::new(&self.macro_dir).join(name);
            let mut content = util::read_file_content(&path).unwrap();
            let max = self.find_max_in_content(&content);
            if max.is_none() {
                panic!("找不到索引，请检查 {} 文件内容是否正确", path.display());
            }
            let max = max.unwrap();
            println!("模板宏内容：");
            println!("{}", content.blue());
            let params = match self.input_params(max) {
                Some(params) => params,
                None => continue,
            };

            for i in 1..=max {
                let from = format!("[{i}]");
                let to = &params[i-1];
                content = content.replace(&from, to);
            }

            println!("宏内容：");
            println!("{}", content.green());
            util::write_to_clipboard(&content).expect(&"写入剪贴板失败".red());
            println!("{}", "已复制到剪贴板".green());
            println!();
        }
    }

    fn input_params(&self, need: usize) -> Option<Vec<String>> {
        let mut params: Vec<String>;
        loop {
            println!("请输入 {need} 个参数，用空格分隔（输入back返回上一级菜单）：");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect(&"读取输入失败".red());
            let input = input.trim();
            if input.is_empty() {
                eprintln!("{}", "输入不能为空，请重新输入".red());
                continue;
            }
            if input == "back" {
                return None;
            }
            params = input.split_whitespace().map(|s| s.to_string()).collect();
            if params.len() != need {
                eprintln!("{}", format!("输入参数数量必须为 {need} 个，请重新输入").red());
                continue;
            }
            break;
        }
        Some(params)
    }

    fn find_max_in_content(&self, content: &str) -> Option<usize> {
        enum State {
            Nothing,
            Left,
            Number,
        }

        let mut state = State::Nothing;
        let mut max = None;
        let mut current = 0;
        let mut indexes = Vec::new();
        for c in content.chars() {
            match state {
                State::Nothing => {
                    if c == '[' {
                        state = State::Left;
                    }
                }
                State::Left | State::Number => {
                    if c == '-' {
                        eprintln!("{}", "不支持负数索引".red());
                        continue;
                    }

                    if c == ']' {
                        state = State::Nothing;
                        if current == 0 {
                            panic!("{}", "索引必须为正整数，请检查模板文件".red());
                        }

                        max = match max {
                            None => {
                                if current != 1 {
                                    panic!("{}", "索引必须从1开始，请检查模板文件".red());
                                }
                                Some(current)
                            }
                            Some(max) => {
                                Some(max.max(current))
                            }
                        };

                        indexes.push(current);
                        current = 0;
                        continue;
                    }

                    if !c.is_numeric() {
                        state = State::Nothing;
                        continue;
                    }
                    state = State::Number;
                    let n = c.to_digit(10).unwrap() as usize;
                    current = current * 10 + n;
                }
            }
        }

        indexes.sort_unstable();
        indexes.dedup();
        for i in 0..indexes.len() - 1 {
            if indexes[i+1] != indexes[i] + 1 {
                panic!("{}", "索引必须连续，请检查模板文件".red());
            }
        }


        if max.is_none() {
            None
        } else if max.unwrap() == 0 {
            None
        } else {
            max
        }
    }

    fn get_macro_txt_names(&self) -> Vec<String> {
        let mut names = vec![];
        for entry in fs::read_dir(&self.macro_dir).unwrap() {
            let entry = entry.unwrap();
            let file_name = entry.file_name();
            names.push(file_name.into_string().unwrap());
        }
        names
    }

    fn check_folder(&self) {
        if !Path::new(&self.macro_dir).is_dir() {
            panic!("没有找到 {} 文件夹", &self.macro_dir.display());
        }
    }
}