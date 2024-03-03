#![allow(unused_imports)]
#![allow(unused_variables)]

use std::io::{ self, stdout, Read, Error };
use crossterm::{
    terminal,
    execute,
    style::{ Print, SetForegroundColor, SetBackgroundColor, ResetColor, Color },
};

fn print_func(str: char) {
    let _a = execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        SetBackgroundColor(Color::Black),
        Print(str.to_string() + "\n"),
        ResetColor
    );
}

fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
}

fn die(e: Error) {
    panic!("错误处理 {}", e);
}

fn main() {
    terminal::enable_raw_mode().ok();
    println!("{:?}", terminal::is_raw_mode_enabled());

    for b in io::stdin().bytes() {
        match b {
            Ok(b) => {
                let c = b as char;
                if c.is_control() {
                    print_func(c);
                } else {
                    print_func(c);
                }
                if b == to_ctrl_byte('q') {
                    // ctrl + q 退出
                    break;
                }
            }
            Err(e) => die(e),
        }
    }
}
