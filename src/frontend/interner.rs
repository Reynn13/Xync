use std::{collections::HashMap};

#[derive(Clone, Copy)]
pub struct StringId(pub usize);

#[derive(Default)]
pub struct Interner {
    map: HashMap<Box<str>, StringId>,

    vec: Vec<*const str>,
}

impl Interner {

    pub fn with_predefined_keywords() -> Self {
        let mut interner = Self::default();
        interner.intern("let"); // ? 0
        interner.intern("mut"); // ? 1
        interner
    }

    pub fn intern(&mut self, string: &str) -> StringId {
        if let Some(&id) = self.map.get(string) {
            return id;
        }

        let boxed_str: Box<str> = string.into();

        let ptr = &*boxed_str as *const str;

        let id = StringId(self.vec.len());

        self.map.insert(boxed_str, id);

        self.vec.push(ptr);

        id
    }

    pub fn resolve(&self, id: StringId) -> &str {
        let ptr = self.vec.get(id.0).expect("Invalid string Id");

        // ! SAFETY: This pointer is still valid if the Interner not yet dropped

        unsafe { &**ptr }
    }
}
