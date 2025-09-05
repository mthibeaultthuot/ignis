pub trait Runtime {
    fn run(&mut self, inputs: &[&[f32]]) -> Vec<f32>;
}
