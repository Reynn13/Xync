/*
    Head: Module `errors`
    Description: Diagnostic implementation and helpers

    @author LightMayo
*/

mod diagnostic;
pub use diagnostic::*;

pub trait XynError {
    fn to_usize(self) -> usize;
    fn message(&self) -> &'static str;
}
