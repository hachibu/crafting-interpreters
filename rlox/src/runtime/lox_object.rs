#[derive(Debug)]
pub enum LoxObject {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil
}
