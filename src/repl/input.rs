use std::io::stdin;
use std::process::exit;

pub struct InputBuffer {
    pub buffer: String,
    pub buffer_length: usize,
    pub input_length: isize,
}

impl InputBuffer {
    pub fn new() -> InputBuffer {
        InputBuffer {
            buffer: String::new(),
            buffer_length: 0,
            input_length: 0
        }
    }

    pub fn init(&mut self) {
        self.buffer = String::new();
        self.buffer_length = 0;
        self.input_length = 0;
    }

    pub fn read_input(&mut self) {
        let byte_read = stdin().read_line(&mut self.buffer).expect("读取失败!");
        self.buffer.pop();
        self.buffer.pop();
        self.input_length = byte_read as isize - 2;
    }

    pub fn do_meta_command(&self) -> Result<(), String> {
        if self.buffer == ".exit" {
            exit(0);
        }else {
            Err("输入指令错误".to_string())
        }
    }

}