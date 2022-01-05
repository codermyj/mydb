use crate::repl::input::InputBuffer;

enum StatementType {
    STATEMENT_INSERT,
    STATEMENT_SELECT,
}

pub enum PrepareResult {
    PREPARE_SUCCESS,
    PREPARE_UNRECOGNIZED_STATEMENT
}

pub struct Statement {
    stmtType: StatementType
}

impl Statement {

    pub fn new() -> Statement {
        Statement{
            stmtType: StatementType::STATEMENT_SELECT
        }
    }

    pub fn prepare_statement(&mut self, input_buffer: &InputBuffer) -> PrepareResult {
        match &input_buffer.buffer[0..6] {
            "insert" => {
                self.stmtType = StatementType::STATEMENT_INSERT;
                PrepareResult::PREPARE_SUCCESS
            },
            "select" => {
                self.stmtType = StatementType::STATEMENT_SELECT;
                PrepareResult::PREPARE_SUCCESS
            },
            _ => PrepareResult::PREPARE_UNRECOGNIZED_STATEMENT
        }
    }

    pub fn execute_statement(&self) {
        match self.stmtType {
            StatementType::STATEMENT_INSERT => println!("这里是一个insert操作"),
            StatementType::STATEMENT_SELECT => println!("这里是一个select操作")
        }
    }
}
