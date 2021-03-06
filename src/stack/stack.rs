use rustc_hash::FxHashMap;

#[derive(Debug)]
pub struct Stack {
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
    pub imma: Vec<i32>,
    pub op: Vec<i64>,
    pub imp_i: i32,
    pub variables: Vec<i64>,
    pub hashes: FxHashMap<u8, Vec<u8>>,
    pub output_stream_st: Vec<String>,
    pub class_stream_st: Vec<String>,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
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
            imma: vec![],
            op: vec![],
            imp_i: 0,
            variables: vec![0; 255],
            hashes: FxHashMap::default(),
            output_stream_st: vec![],
            class_stream_st: vec![],
        }
    }
}
