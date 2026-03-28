use std::{collections::HashMap, rc::Rc};

use oxrdf::Term;

#[derive(Debug, Default)]
pub struct TermIndex {
    /// Maps an RDF term to a corresponding id.
    str_index: HashMap<Rc<Term>, usize>,
    /// Maps an id to a corresponding RDF term.
    int_index: HashMap<usize, Rc<Term>>,
}

impl TermIndex {
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts
    pub fn insert(&mut self, term: Term) -> usize {
        match self.str_index.get(&term) {
            Some(id) => *id,
            None => {
                let rc_term = Rc::new(term);
                let id = self.str_index.len();

                self.str_index.insert(rc_term.clone(), id);
                self.int_index.insert(id, rc_term);

                id
            }
        }
    }

    /// Removes a term from the index, returning the term with the id if the id was previously in the index.
    pub fn remove(&mut self, id: &usize) -> Option<Rc<Term>> {
        self.int_index.remove(id).inspect(|term| {
            self.str_index.remove(term);
        })
    }

    /// Returns a reference to the term corresponding to the id.
    pub fn get(&self, id: &usize) -> Option<Rc<Term>> {
        self.int_index.get(id).cloned()
    }

    /// Returns true if the term corresponding to the id exists and is a named node.
    pub fn is_named_node(&self, id: &usize) -> bool {
        self.int_index
            .get(id)
            .is_some_and(|term| term.is_named_node())
    }

    /// Returns true if the term corresponding to the id exists and is a blank node.
    pub fn is_blank_node(&self, id: &usize) -> bool {
        self.int_index
            .get(id)
            .is_some_and(|term| term.is_blank_node())
    }

    /// Returns true if the term corresponding to the id exists and is a literal.
    pub fn is_literal(&self, id: &usize) -> bool {
        self.int_index.get(id).is_some_and(|term| term.is_literal())
    }
}
