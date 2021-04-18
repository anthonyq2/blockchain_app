#[derive(Debug)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f32,
}