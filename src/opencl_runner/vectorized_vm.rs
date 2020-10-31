pub struct VectorizedVM<'a> {
    // each VM has its own WASI state tracking object
    temp: Vec<&'a str>,
}

impl<'a> VectorizedVM<'a> {
    pub fn new() -> VectorizedVM<'a> {
        VectorizedVM {
            temp: vec!(),
        }
    }
}