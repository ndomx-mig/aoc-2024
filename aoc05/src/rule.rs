#[derive(Debug, serde::Deserialize)]
pub struct Rule {
    pub first: usize,
    pub second: usize,
}
