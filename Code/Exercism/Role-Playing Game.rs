// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]
pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}
impl Player {
    pub fn revive(&self) -> Option<Player> {
        match self.health {
            0 => {
                let player = Player {
                    health: 100,
                    mana: match self.level {
                        0..=9 => None,
                        _ => Some(100),
                    },
                    level: self.level,
                };
                Some(player)
            }
            _ => None,
        }
    }
    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        let mut damage = 2 * mana_cost; 
        match self.mana {
            Some(x) => {
                if x < mana_cost {
                    damage = 0;
                }
                
                else { 
                    self.mana = Some(x - mana_cost);
                }
            }
            None => {
                if self.health <= mana_cost {
                    self.health = 0;
                } 
                
                else { 
                    self.health -= mana_cost;
                }
                
                damage = 0;
            }
        }
        damage
    }
}
