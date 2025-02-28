
pub fn bigger_num<'a>(x:&'a u32, y:&'a u32) -> &'a u32 {
    if x > y {
        x
    } else {
        y
    }
}

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


