use minicbor::Encode;

#[derive(Encode, Clone, Debug)]
pub struct PackedValue {
    #[n(0)]
    pub value: String,
    #[n(1)]
    pub pages: Vec<usize>,
}

fn main() {
    println!("Hello, world!");
}
