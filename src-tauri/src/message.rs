#[derive(Clone, Copy)]
pub enum Message {
  Close,
  Update(u32),
  CounterEnd,
  BlockerEnd,
}
