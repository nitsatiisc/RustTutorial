

pub struct VecVec<'a> {
    pub vec_vec: Vec<&'a Vec<u32>>,
}

impl<'a> VecVec<'a> {
    pub fn new() -> Self {
        VecVec {
            vec_vec: vec![],
        }
    }

    pub fn add_vec<'c:'a>(&mut self, new_vec:&'c Vec<u32>)  {
      self.vec_vec.push(new_vec)
    }
}


