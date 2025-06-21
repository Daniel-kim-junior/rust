use rand::Rng;

use crate::character::Character;

pub struct Monster {
  name: String,
  hp: u32,
  level: u8,
  exp: u32,
  attack_power: u32
}


impl Monster {
    pub fn new(name: &str) -> Self {
        Monster {
            name: String::from(name),
            hp: 100,
            level: 1,
            exp: 50,
            attack_power: rand::thread_rng().gen_range(5..15), // 랜덤 공격력
        }
    }

    pub fn print_status(&self) {
        println!("Monster Name: {}", self.name);
        println!("Monster HP: {}", self.hp);
        println!("Monster Level: {}", self.level);
    }

    pub fn delete(self) {
        println!("몬스터가 삭제되었습니다");
    }
    
    pub fn experience(&self) -> u32 {
        self.exp
    }

 
}


impl Character for Monster {
    fn is_dead(&self) -> bool {
        self.hp == 0
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn attack(&mut self, target: &mut dyn Character) {
        target.take_damage(self.attack_power);
    }
    fn take_damage(&mut self, amount: u32) {
        if self.hp > amount {
            self.hp -= amount;
        } else {
            self.hp = 0;
        }
    }
    fn experience(&self) -> u32 {
        self.exp
    }
}