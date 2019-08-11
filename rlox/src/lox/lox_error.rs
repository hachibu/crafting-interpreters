use lox::*;
use syntax::*;
use std::fmt;
use yansi::Color;

#[derive(Debug, PartialEq)]
pub struct LoxError<'a> {
    pub ty: LoxErrorTy,
    pub message: &'a str,
    pub source: &'a str,
    pub source_file: &'a Option<String>,
    pub position: Position
}

impl<'a> LoxError<'a> {
    pub fn new(ty: LoxErrorTy, message: &'a str, source: &'a str, source_file: &'a Option<String>, position: Position) -> LoxError<'a> {
        LoxError { ty, message, source, source_file, position }
    }
}

impl<'a> fmt::Display for LoxError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err_lines: Vec<&str> =
            self.source.get(0..self.position.offset).unwrap_or("")
                       .lines()
                       .collect();

        let (err_line, err_col) =
            if err_lines.len() == 0 {
                (0, 0)
            } else {
                let err_line = err_lines.len() - 1;
                let err_col = err_lines.get(err_line).unwrap().len();
                (err_line, err_col)
            };

        let lines: Vec<&str> = self.source.split('\n').collect();
        let curr_line: &str = lines.get(err_line).unwrap();
        let prev_line: &str =
            if err_line == 0 {
                ""
            } else {
                lines.get(err_line - 1).unwrap()
            };
        let curr_line_num: String = format!("{} | ", err_line + 1);
        let curr_line_ptr: String = format!(
            "{}^",
            "-".repeat(curr_line_num.len() + err_col)
        );
        let prev_line_num: String = format!(
            "{}| ",
            " ".repeat(curr_line_num.len() - 2)
        );
        let file_line_num: String = format!(
            "{}> ",
            "-".repeat(curr_line_num.len() - 2)
        );

        let pretty_err_msg: String = format!("{error}: {err_msg}
{file_line_num}{file}{err_line}:{err_col}
{prev_line_num}{prev_line}
{curr_line_num}{curr_line}
{curr_line_ptr}",
            file = match self.source_file {
                Some(ref s) => format!("{}:", s),
                None => String::from("")
            },
            file_line_num = Color::Blue.paint(file_line_num),
            error = Color::Red.paint(&self.ty),
            err_msg = self.message,
            err_line = err_line + 1,
            err_col = err_col + 1,
            prev_line_num = Color::Blue.paint(prev_line_num),
            prev_line = prev_line,
            curr_line_num = Color::Blue.paint(curr_line_num),
            curr_line = curr_line,
            curr_line_ptr = Color::Red.paint(curr_line_ptr)
        );

        write!(f, "{}", pretty_err_msg.to_string())
    }
}
