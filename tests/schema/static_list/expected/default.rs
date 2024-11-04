pub type Array = ArrayType;
#[derive(Debug, Clone)]
pub struct ArrayType {
    pub item: [IntType; 5usize],
}
pub type IntType = i32;
