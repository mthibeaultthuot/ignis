use ignis_macro::ignis;

#[ignis]
pub fn Add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let k = Add::to_ir();
        println!("{:#?}", k);
    }
}
