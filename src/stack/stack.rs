#[derive(Debug)]
pub struct StackMachine {
    pub i_st0: i32,
    pub i_st1: i32,
    pub i_st2: i32,
    pub i_st3: i32,
    pub f_st0: f32,
    pub f_st1: f32,
    pub f_st2: f32,
    pub f_st3: f32,
    pub a_st0: Vec<i32>,
    pub a_st1: Vec<i32>,
    pub a_st2: Vec<i32>,
    pub a_st3: Vec<i32>,
    pub imm: Vec<i64>,
    pub op: Vec<i64>,
    pub imp_i: i32,
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
            a_st0: vec![],
            a_st1: vec![],
            a_st2: vec![],
            a_st3: vec![],
            imm: vec![],
            op: vec![],
            imp_i: 0,
        }
    }
}
