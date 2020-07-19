#[derive(Debug)]
pub struct StackMachine {
    pub i_st0: i8,
    pub i_st1: i8,
    pub i_st2: i8,
    pub i_st3: i8,
    pub f_st0: f32,
    pub f_st1: f32,
    pub f_st2: f32,
    pub f_st3: f32,
    pub imm: Vec<u8>,
    pub op: Vec<u8>,
    pub imp_i: u8,
}

impl StackMachine {
    pub fn new() -> Self {
        StackMachine {
            i_st0: 0,
            i_st1: 0,
            i_st2: 0,
            i_st3: 0,
            f_st0: 0f32,
            f_st2: 0f32,
            f_st1: 0f32,
            f_st3: 0f32,
            imm: vec![],
            op: vec![],
            imp_i: 0,
        }
    }
}
