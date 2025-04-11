// use rand::seq::IndexedRandom;

pub struct Agent<T> {
    name: String,
    atk: u32,
    def: u32,
    hp: u32,
    // ここでジェネリック型を定義
    skill: Option<T>,
}

impl<T> Agent<T> {
    pub fn new(name: String, atk: u32, def: u32, hp: u32) -> Self {
        Self {
            name,
            atk,
            def,
            hp,
            skill: None,
        }
    }
}
impl<T> Agent<T> {
    fn set_skill(&mut self, skill: T) {
        self.skill = Some(skill);
    }
}

// 戦闘フロー
impl<T> Agent<T> {
    fn attack(&self, target: &mut Self) {
        let damage = if self.atk > target.def {
            self.atk - target.def
        } else {
            0
        };
        target.hp = target.hp.saturating_sub(damage);
        println!(
            "{} attacks {} for {} damage! {}'s HP: {}",
            self.name, target.name, damage, target.name, target.hp
        );
    }
}

pub fn iterate_turn<T>(agents: &mut [Agent<T>]) {
    /* ---------------------------------------
    戦闘イテレーション
    --------------------------------------- */
    assert_eq!(agents.len(), 2);

    'outer: loop {
        for i in 0..agents.len() {
            let target_index = (i + 1) % agents.len();

            // split_at_mut を使用して、ベクターを2つの可変な非重複スライスに分割
            let (first, second) = agents.split_at_mut(1);

            let (attacker, target) = if i < target_index {
                (&first[0], &mut second[0])
            } else {
                (&second[0], &mut first[0])
            };

            attacker.attack(target);
            if target.hp == 0 {
                println!("{} wins!", attacker.name);
                break 'outer;
            }
        }
    }
}
