#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
  pub from: String,
  pub to: String,
  pub amount: u64,
}

