pub struct Expression {
    pub term1: i32,
    pub term2: i32,
    pub operation: fn(i32, i32) -> Option<i32>,
}
