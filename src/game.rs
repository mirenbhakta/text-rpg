pub mod stats;

use std::ops::Range;
use crate::Rand;
use rand::prelude::*;

use stats::*;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Game {

}

pub struct Player {
    health: i32,
    mana: i32,
    energy_shield: i32,

    pub stats: StatMap,

    main_hand: Item,
    off_hand: Item,

    helmet: Item,
    body_armour: Item,
    gloves: Item,
    boots: Item,
    left_ring: Item,
    right_ring: Item,
    amulet: Item,
}

/*
// attacks per second = 1 / (attack_time / 1000)
let weapon_attack_time = 1000_i32;
*/

const CRIT_CHANCE_MAX_VALUE: i32 = 100 * 100; 

impl Player {
    pub fn new() -> Self {
        let mut stats = StatMap::new();
        for stat in ALL_STATS {
            stats.add(*stat, 0);
        }
        Self {
            health: 100,
            mana: 100,
            energy_shield: 100,
            stats,
            main_hand: Item::default(),
            off_hand: Item::default(),
            helmet: Item::default(),
            body_armour: Item::default(),
            gloves: Item::default(),
            boots: Item::default(),
            left_ring: Item::default(),
            right_ring: Item::default(),
            amulet: Item::default(),
        }
    }

    pub fn default_attack_test(&mut self, rand: &mut Rand) -> String {
        // temp weapon values
        let wep_flat_physical_local_min = 15;
        let wep_flat_physical_local_max = 24;
        let wep_crit_chance_local = 500; // 5.00% chance to crit

        let accuracy = self.stats.get(Accuracy);
        let accuracy_inc = self.stats.get(AccuracyInc);

        let accuracy_final = accuracy * (100 + accuracy_inc) / 100;

        // compare accuracy to enemy evasion
        // for now just assume 100% hit chance
        if accuracy_final < 0 {
            // attack missed
            return String::new();
        }

        let crit_chance = wep_crit_chance_local + self.stats.get(CritChance);
        let crit_chance_inc = self.stats.get(CritChanceInc);
        let crit_chance_final = crit_chance * (100 + crit_chance_inc) / 100;

        // for rolling crit, generate random number between 0 and CRIT_CHANCE_MAX_VALUE
        let crit_roll = rand.gen_range(0..=CRIT_CHANCE_MAX_VALUE);
        
        let crit_damage_bonus_if_crit = 50 + self.stats.get(CritDamageBonus);
        let crit_damage_bonus = if crit_roll < crit_chance_final {
            // player has inherit 50% crit damage bonus
            crit_damage_bonus_if_crit
        } else {
            // crit miss
            0
        };

        let increased_dmg =
            self.stats.get(DamageInc) +
            self.stats.get(PhysicalInc) +
            self.stats.get(AttackInc);

        let wep_flat_physical_local_min_final = wep_flat_physical_local_min * (100 + increased_dmg) / 100;
        let wep_flat_physical_local_max_final = wep_flat_physical_local_max * (100 + increased_dmg) / 100;
        
        // use rand to generate a random number between wep_flat_physical_local_min_final and wep_flat_physical_local_max_final
        let weapon_physical_damage_final = rand.gen_range(wep_flat_physical_local_min_final..=wep_flat_physical_local_max_final);

        let weapon_critical_physical_damage_final = weapon_physical_damage_final * (100 + crit_damage_bonus) / 100;

        let wep_critical_physical_local_min_final = wep_flat_physical_local_min_final * (100 + crit_damage_bonus_if_crit) / 100;
        let wep_critical_physical_local_max_final = wep_flat_physical_local_max_final * (100 + crit_damage_bonus_if_crit) / 100;

        format!("Weapon deals {} to {} physical damage with a base critical hit chance of {:.2}.
Base Critical Hit Chance is {} after player modifiers, and is Increased by {}% to get a final critical hit chance of {:.2}.
If the hit is a Critical Hit, the Default attack's damage will be increased by {}% Critical Damage Bonus. 
Default attack will deal {} to {} physical damage with Increased Damage modifiers of {}% Damage, {}% Attack Damage, and {}% Physical Damage.
If the hit is a Critical Hit, Default attack will deal {} to {} physical damage.

With random rolls, the default attack did {} physical damage, with a {}.", 
            wep_flat_physical_local_min, wep_flat_physical_local_max, wep_crit_chance_local as f32 / 100.0,
            crit_chance as f32 / 100.0, self.stats.get(CritChanceInc), crit_chance_final as f32 / 100.0,
            crit_damage_bonus_if_crit,
            wep_flat_physical_local_min_final, wep_flat_physical_local_max_final, self.stats.get(DamageInc), self.stats.get(PhysicalInc), self.stats.get(AttackInc),
            wep_critical_physical_local_min_final, wep_critical_physical_local_max_final,
            weapon_critical_physical_damage_final, if crit_roll < crit_chance_final {"Critical Hit"} else {"Non-Critical Hit"},
        )
    }
}

pub struct PassiveTree {

}

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct Item {

}

pub struct Modifier {
    kind: Stat,
    value: Range<i32>,
}
