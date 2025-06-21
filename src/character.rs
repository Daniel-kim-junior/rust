pub trait Character {
  fn is_dead(&self) -> bool;
  fn name(&self) -> &str;
  fn attack(&mut self, target: &mut dyn Character);
  fn take_damage(&mut self, amount: u32);
  fn experience(&self) -> u32 {
    0 // 기본 구현, 필요시 오버라이드 가능
  }
}