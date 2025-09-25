use crate::SSAVar;
use std::collections::HashMap;

/// Context for the Static Single Assignment (SSA)
///
/// The main goal purpose of this context is to manage
/// variables in a way that each variable is assigned
/// only once, ensuring that each variable has a unique
/// identifier.
///
/// Example :
/// ```text %0 = a + b
/// %1 = %0 * c
/// %2 = %1 - d```
///
/// where %0, %1, %2 are SSA variables.
#[derive(Debug, Clone)]
pub struct LowerCtx {
    next_ssa: u32,
    vars: HashMap<String, SSAVar>,
}

impl LowerCtx {
    pub fn new() -> Self {
        Self {
            next_ssa: 0,
            vars: HashMap::new(),
        }
    }

    pub fn fresh(&mut self) -> SSAVar {
        let id = self.next_ssa;
        self.next_ssa += 1;
        SSAVar(id)
    }

    pub fn get_or_create(&mut self, name: &str) -> SSAVar {
        if let Some(v) = self.vars.get(name) {
            *v
        } else {
            let id = self.fresh();
            self.vars.insert(name.to_string(), id);
            id
        }
    }
}
