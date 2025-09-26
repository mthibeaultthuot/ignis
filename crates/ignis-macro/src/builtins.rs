#[allow(dead_code)]
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Builtin {
    ThreadIdxX,
    ThreadIdxY,
    ThreadIdxZ,
    BlockIdxX,
    BlockIdxY,
    BlockIdxZ,
    BlockDimX,
    BlockDimY,
    BlockDimZ,
    GridDimX,
    GridDimY,
    GridDimZ,
    LaneId,
    WarpSize,
}
