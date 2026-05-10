use crate::*;

impl Trunc for num_bigfloat::BigFloat {
    fn trunc(self) -> Self {
        self.floor()
    }
}
impl Fract for num_bigfloat::BigFloat {
    fn fract(self) -> Self {
        self - self.trunc()
    }
}
