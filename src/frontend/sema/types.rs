
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Type {
    I32,
    Unbound(usize), // id
}