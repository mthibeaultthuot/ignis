use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SSAVar(pub u32);

impl fmt::Debug for SSAVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "%{}", self.0)
    }
}

impl fmt::Display for SSAVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "v{}", self.0)
    }
}

impl Hash for SSAVar {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct BufferId(pub u32);

impl fmt::Debug for BufferId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "b{}", self.0)
    }
}
impl Hash for BufferId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum DType {
    I32,
    I64,
    U32,
    F16,
    F32,
    F64,
    Bool,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct Type {
    pub scalar: DType,
    pub lanes: u8,
}

#[allow(dead_code)]
impl Type {
    pub const I32: Self = Self {
        scalar: DType::I32,
        lanes: 1,
    };
    pub const F32: Self = Self {
        scalar: DType::F32,
        lanes: 1,
    };
    pub const BOOL: Self = Self {
        scalar: DType::Bool,
        lanes: 1,
    };
}
