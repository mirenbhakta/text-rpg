
pub use Stat::*;

use crate::sparse::SparseVec;

macro_rules! make_stat_enum {
    (enum $name: ident {
        $($variant: ident),*,
    }) => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        #[repr(u16)]
        pub enum $name {
            $($variant),*,
        }

        pub const ALL_STATS: &[Stat] = &[$(Stat::$variant),*];

        impl $name {
            pub fn name(&self) -> &'static str {
                match self {
                    $(Self::$variant => stringify!($variant),)*
                }
            }
        }
    };
}

make_stat_enum! {
    enum Stat {
        MaxHealth, // health, if health == max health, you are at full health, if health == 0, you are dead
        MaxHealthInc,

        MaxMana, // mana, used by most skills, skills drain mana over time according to their mana cost over the duraction of the skill's action time
        MaxManaInc,

        MaxEnergyShield, // energy shield, damage is taken by energy shield before health, chaos damage bypasses energy shield
        MaxEnergyShieldInc,

        MaxSpirit, // persistent skills will reserve spirit to maintain their effects

        Evasion, // chance to avoid damage from attacks and spell projectiles
        EvasionInc,

        Armour, // reduces physical damage taken based on its value
        ArmourInc,

        Accuracy, // chance to hit, is compared to enemy's evasion
        AccuracyInc,

        LightningResist,
        ColdResist,
        FireResist,
        ChaosResist,

        // chance to critically hit, if a hit is a critical hit its damage will be multiplied by the CritDamage stat
        CritChance, // flat crit chance
        CritChanceInc,
        CritChanceLocal,
        CritChanceLocalInc,
        CritDamageBonus,

        ActionSpeed,       // animation speed, makes you faster or slower at everything
        MoveSpeed,         // the speed at which you can move
        SkillSpeed,        // affects most speed related stats that skills use
        AttackSpeed,       // the speed of attack skills
        SpellSpeed,        // the speed of spell skills
        TrapThrowingSpeed, // the speed at which you can throw traps

        ExpireSpeed, // makes various time based effects last shorter or longer
        Cooldown,    // makes cooldowns last shorter or longer

        DamageInc, // generic damage, cannot be dealt but can scale all types of damage

        // damage types
        Physical, // damage that would physically harm or destroy your body
        PhysicalInc,
        PhysicalLocal,
        PhysicalLocalInc,

        Lightning, // like touching a live AC wire or a tesla coil
        LightningInc,

        Cold, // touching extremely cold things or being in a cold environment
        ColdInc,

        Fire, // being on fire lol, or touching something super hot
        FireInc,

        Chaos, // the opposite of life, themed around the occult or rot
        ChaosInc,

        // damage sources
        AttackInc,         // almost always dealt by actual weapons
        SpellInc,          // dealt by magic spells
        DamageOverTimeInc, // dealt by ailments or debuffs

        StunChance, // stun can be caused by all hits
        StunThreshold,

        // status ailments
        BleedInc,    // deals physical damage over time
        BleedChance, // chance to inflict bleed

        ShockInc, // makes you take increased damage based on its effectiveness
        ShockChance, // chance to inflict shock, its effectiveness is based on lightning damage

        ChillInc, // reduces action speed based on its effectiveness
        ChillChance, // chance to inflict chill, its effectiveness is based on cold damage

        FreezeInc, // sets action speed to 0
        FreezeChance, // chance to freeze

        IgniteInc, // deals fire damage over time
        IgniteChance, // chance to inflict ignite

        PoisonInc, // deals chaos damage over time
        PoisonChance, // chance to inflict poison

        // debuffs
        IntimidateChance, // increased attack damage taken by 20%
        UnnerveChance,    // increased spell damage taken by 20%

        MaimChance,   // reduces movement speed by 20%, only caused by attacks
        HinderChance, // reduces movement speed by 20%, only caused by spells

        BlindChance, // 20% less evasion and accuracy
    }
}

/*
Burning, // deals fire damage over time, but is not the same as ignite
Withered, // increased chaos damage taken by 20%

Secondary, // damage of secondary effects like corpse explosions
*/

impl Stat {
    fn idx(self) -> u16 {
        self as u16
    }
}

#[derive(Default)]
pub struct StatMap {
    map: SparseVec<i32>,
}

impl StatMap {
    pub fn new() -> Self {
        Self {
            map: SparseVec::new(2048),
        }
    }

    pub fn get(&self, stat: Stat) -> i32 {
        self.map.get(stat.idx()).copied().unwrap_or(0)
    }

    pub fn debug_get_mut(&mut self, stat: Stat) -> &mut i32 {
        self.map.get_mut(stat.idx()).unwrap()
    }

    pub fn add(&mut self, stat: Stat, value: i32) {
        self.map.entry(stat.idx())
            .and_modify(|val| *val += value)
            .or_insert(value);
    }

    pub fn reset(&mut self, stat: Stat) {
        self.map.remove(stat.idx());
    }

    pub fn reset_all(&mut self) {
        self.map.clear();
    }

}
