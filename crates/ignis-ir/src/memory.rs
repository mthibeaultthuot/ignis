use core::fmt;

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub enum AtomicOp {
    Add,
    Sub,
    Min,
    Max,
    And,
    Or,
    Xor,
    Exchange,
    CompareExchange,
}

impl core::fmt::Display for AtomicOp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            AtomicOp::Add => "atomic_add",
            AtomicOp::Sub => "atomic_sub",
            AtomicOp::Min => "atomic_min",
            AtomicOp::Max => "atomic_max",
            AtomicOp::And => "atomic_and",
            AtomicOp::Or => "atomic_or",
            AtomicOp::Xor => "atomic_xor",
            AtomicOp::Exchange => "atomic_exchange",
            AtomicOp::CompareExchange => "atomic_cas",
        };
        f.write_str(s)
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub enum AddressSpace {
    Global,
    Shared,
    Local,
    Const,
}

impl fmt::Display for AddressSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            AddressSpace::Global => "global",
            AddressSpace::Shared => "shared",
            AddressSpace::Local => "local",
            AddressSpace::Const => "const",
        })
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub enum BarrierScope {
    ThreadGroup,
    Device,
}

impl fmt::Display for BarrierScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            BarrierScope::ThreadGroup => "threadgroup",
            BarrierScope::Device => "device",
        })
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub enum MemFenceScope {
    ThreadGroup,
    Device,
    System,
}

impl fmt::Display for MemFenceScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            MemFenceScope::ThreadGroup => "threadgroup",
            MemFenceScope::Device => "device",
            MemFenceScope::System => "system",
        })
    }
}
