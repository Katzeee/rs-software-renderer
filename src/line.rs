use crate::shader::*;

#[derive(Clone, Copy)]
pub struct Line {
    pub start: Vertex,
    pub end: Vertex,
}
impl Line {
    pub fn new(start: Vertex, end: Vertex) -> Self {
        Self {
            start: start,
            end: end,
        }
    }
}
