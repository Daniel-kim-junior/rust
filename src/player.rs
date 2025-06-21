use crate::{character::Character};

pub struct Player {
  name: String,
  hp: u32,
  level: u8,
  ex: u32,
  attack_power: u32,
}

impl Player {
    pub fn print_status(&self) {
        println!("Player Name: {}", self.name);
        println!("Player HP: {}", self.hp);
        println!("Player Level: {}", self.level);
    }


    pub fn new(name: &str) -> Self {
        Player {
            name: String::from(name),
            hp: 100,
            level: 1,
            ex: 0,
            attack_power: 10,
        }
    }

   

    fn gain_exp(&mut self, amount: u32) {
      self.ex += amount;
      self.level = (self.ex / 100) as u8; // 예시로 100 경험치마다 레벨업
    }
}

impl Character for Player {
    fn is_dead(&self) -> bool {
        self.hp == 0
    }
    fn name(&self) -> &str {
        &self.name
    }

    fn attack(&mut self, target: &mut dyn Character) {
      target.take_damage(self.attack_power);
      if target.is_dead() {
          self.gain_exp(target.experience());
      }
    }

    fn experience(&self) -> u32 {
        self.ex
    }

    fn take_damage(&mut self, amount: u32) {
        if self.hp > amount {
            self.hp -= amount;
        } else {
            self.hp = 0;
        }
    }
}