// use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicUsize, Ordering};

use super::node::Node;
use crate::front::semantic::SemanticContext;

static IDENTIFIER_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct Identifier {
    pub name: String,
    pub id: usize,
}

impl Identifier {
    pub fn new(name: String) -> Self {
        let id = IDENTIFIER_COUNTER.fetch_add(1, Ordering::Relaxed);
        Identifier { name, id }
    }
}

/* Might be redundant in the near future

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Identifier {}

impl Hash for Identifier {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
*/

impl Node for Identifier {
    fn push_child(&mut self, _c: Box<dyn Node>) {
        panic!("Node: `Identifier` can't bear children!");
    }

    fn display(&self, indentation: usize) {
        println!(
            "{:>width$}-> Identifier: `{}` : {}",
            "",
            self.name,
            self.id,
            width = indentation
        );
    }

    fn analyze(&self, ctx: &mut SemanticContext) -> Result<(), String> {
        // Ensure the identifier exists in the current or parent scopes
        if ctx.lookup(self.id).is_none() {
            return Err(format!("Undefined identifier: {}", self.name));
        }
        Ok(())
    }

    fn ir(&self, ctx: &mut crate::middle::ir::IRContext) -> Vec<crate::middle::ir::IRInstruction> {
        Vec::new()
    }
}

impl From<String> for Identifier {
    fn from(name: String) -> Self {
        Identifier::new(name)
    }
}

impl From<&str> for Identifier {
    fn from(name: &str) -> Self {
        Identifier::new(name.to_string())
    }
}
