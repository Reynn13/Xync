/*
    Head: Module `frontend`
    Description: Module declarations and uses for it's submodules

    @author LightMayo
*/

mod lexers;
pub use lexers::*;

mod parsers;
pub use parsers::*;

mod sema;
pub use sema::*;
