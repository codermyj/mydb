mod repl;
mod vm;
mod utils;


use std::io::{stdout, Write};
use std::process::exit;
use repl::*;
use repl::input::*;
use vm::statement::*;

fn main() {
    let mut input_buffer = InputBuffer::new();
    loop {
        input_buffer.init();
        print_prompt();
        stdout().flush();
        input_buffer.read_input();

        if input_buffer.input_length == 0 {
            continue;
        }

        if &input_buffer.buffer[0..1] == "." {
            match input_buffer.do_meta_command() {
                Ok(_) => continue,
                Err(b) => {
                    println!("{} {}", b, input_buffer.buffer);
                    continue;
                }
            }
        }else if input_buffer.input_length < 6 {
            println!("输入指令错误!");
            continue;
        }

        let mut statement = Statement::new();
        match statement.prepare_statement(&input_buffer) {
            PrepareResult::PREPARE_SUCCESS => (),
            PrepareResult::PREPARE_UNRECOGNIZED_STATEMENT => {
                println!("未能识别的句首关键字 {}", input_buffer.buffer);
                continue;
            },
            PrepareResult::PREPARE_SYNTAX_ERROR => {
                println!("语法错误!");
                continue;
            }
        }
        statement.execute_statement();
        println!("执行完成!");
    }

}
