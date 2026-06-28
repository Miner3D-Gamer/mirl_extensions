/// More functionality for the [`bool`] type
pub trait BoolExtension {
    /// Returned the inverted value if the condition is true
    fn invert_if(self, condition: impl Fn(bool) -> bool) -> bool;
    /// Set self to the inverted value if the condition is true
    fn set_invert_if(&mut self, condition: impl Fn(bool) -> bool);
}
impl BoolExtension for bool {
    fn invert_if(self, condition: impl Fn(Self) -> Self) -> bool {
        if condition(self) { !self } else { self }
    }
    fn set_invert_if(&mut self, condition: impl Fn(Self) -> Self) {
        *self = self.invert_if(condition);
    }
}
