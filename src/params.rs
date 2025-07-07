use unicorn_engine::RegisterX86;

#[derive(Debug, Clone)]
pub enum Parameter<'a> {
    Push32(u32),
    Push64(u64),
    PushStr(&'a str),
    PushString(String),
    PushBuf(usize),
    // TODO: Pod objects?
    Reg32(RegisterX86, u32),
    Reg64(RegisterX86, u64),
    RegStr(RegisterX86, &'a str),
    RegString(RegisterX86, String),
    RegBuf(RegisterX86, usize),
    RegBytes(RegisterX86, &'a [u8]),
    MovReg(RegisterX86, RegisterX86),
}

#[derive(Debug, Clone)]
pub struct Parameters<'a> {
    pub(crate) entries: Vec<Parameter<'a>>,
}

impl<'a> Parameters<'a> {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn push_u32(mut self, value: u32) -> Self {
        self.entries.push(Parameter::Push32(value));
        self
    }

    pub fn push_u64(mut self, value: u64) -> Self {
        self.entries.push(Parameter::Push64(value));
        self
    }

    pub fn push_str(mut self, value: &'a str) -> Self {
        self.entries.push(Parameter::PushStr(value));
        self
    }

    pub fn push_string(mut self, value: String) -> Self {
        self.entries.push(Parameter::PushString(value));
        self
    }

    pub fn push_buf(mut self, size: usize) -> Self {
        self.entries.push(Parameter::PushBuf(size));
        self
    }

    pub fn reg_u32(mut self, reg: RegisterX86, value: u32) -> Self {
        self.entries.push(Parameter::Reg32(reg, value));
        self
    }

    pub fn reg_u64(mut self, reg: RegisterX86, value: u64) -> Self {
        self.entries.push(Parameter::Reg64(reg, value));
        self
    }

    // vvv ADD THIS METHOD vvv
    pub fn reg_bytes(mut self, reg: RegisterX86, value: &'a [u8]) -> Self {
        self.entries.push(Parameter::RegBytes(reg, value));
        self
    }

    pub fn reg_str(mut self, reg: RegisterX86, value: &'a str) -> Self {
        self.entries.push(Parameter::RegStr(reg, value));
        self
    }

    pub fn reg_string(mut self, reg: RegisterX86, value: String) -> Self {
        self.entries.push(Parameter::RegString(reg, value));
        self
    }

    pub fn reg_buf(mut self, reg: RegisterX86, size: usize) -> Self {
        self.entries.push(Parameter::RegBuf(reg, size));
        self
    }

    pub fn mov_reg(mut self, from: RegisterX86, to: RegisterX86) -> Self {
        self.entries.push(Parameter::MovReg(from, to));
        self
    }
}

impl<'a> Default for Parameters<'a> {
    fn default() -> Self {
        Self::new()
    }
}
