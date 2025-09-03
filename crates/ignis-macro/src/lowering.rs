use std::collections::HashMap;

use ignis_ir::SSAVar;

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
