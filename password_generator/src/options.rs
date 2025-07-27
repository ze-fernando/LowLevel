pub struct Options {
    pub lowercase: bool,
    pub uppercase: bool,
    pub number: bool,
    pub symbols: bool,
}

impl Options {
    pub fn any_enabled(&self) -> bool {
        self.lowercase || self.uppercase || self.number || self.symbols
    }
}
