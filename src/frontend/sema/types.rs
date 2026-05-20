/*
    Head: Semantic type
    Description: Semantic type's APIs and Items

    @author LightMayo
*/

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Type {
    I32,
    Unbound(usize), // id
}
