/// location of a symbol in code
#[derive(Debug, Clone)]
pub struct Loc {
    pub file: String,
    pub line: usize,
    pub column: usize,
}

impl Loc {
    /// &'static str as name to ensure only debug use
    pub fn dummy(foo_file: &'static str) -> Self {
        Self {
            file: foo_file.into(),
            line: 0,
            column: 0,
        }
    }

    pub fn new(file: String, line: usize, column: usize) -> Self {
        Self { file, line, column }
    }
}
