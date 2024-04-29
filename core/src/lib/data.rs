use std::collections::HashMap;

use serde::{ Deserialize, Serialize };

pub type TraitId = String;
pub type ChampionId = String;

#[derive(Clone)]
pub struct Trait {
    pub name: TraitId,
    pub thresholds: Vec<u8>,
}

impl Trait {
    pub fn new(name: &str, thresholds: Vec<u8>) -> Self {
        Self {
            name: name.into(),
            thresholds,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Champion {
    pub name: ChampionId,
    pub cost: u8,
    pub range: u8,
    pub traits: Vec<TraitId>,
    pub uses_ap: bool,
}

impl Champion {
    pub fn new(
        cost: u8,
        name: &str,
        traits: Vec<&str>,
        range: u8,
        uses_ap: bool
    ) -> Self {
        Self {
            name: name.into(),
            cost,
            range,
            traits: Vec::from_iter(
                traits.iter().map(|s| String::from(*s))
            ),
            uses_ap,
        }
    }
}

pub struct GameData {
    pub traits: HashMap<TraitId, Trait>,
    pub champions: HashMap<ChampionId, Champion>,

    pub champions_by_trait: HashMap<TraitId, Vec<ChampionId>>,
}

impl GameData {
    pub fn new() -> Self {
        let traits = HashMap::from_iter(
            [
                Trait::new("Altruist", [2, 3, 4].into()),
                Trait::new("Arcanist", [2, 4, 6, 8].into()),
                Trait::new("Behemoth", [2, 4, 6].into()),
                Trait::new("Bruiser", [2, 4, 6, 8].into()),
                Trait::new("Dragonlord", [2, 3, 4, 5].into()),
                Trait::new("Dryad", [2, 4, 6].into()),
                Trait::new("Duelist", [2, 4, 6, 8].into()),
                Trait::new("Fated", [3, 5, 7, 10].into()),
                Trait::new("Fortune", [3, 5].into()),
                Trait::new("Ghostly", [2, 4, 6, 8].into()),
                Trait::new("Heavenly", [2, 3, 4, 5, 6, 7].into()),
                Trait::new("Inkshadow", [3, 5, 7].into()),
                Trait::new("Invoker", [2, 4, 6].into()),
                Trait::new("Mythic", [3, 5, 7, 10].into()),
                Trait::new("Porcelain", [2, 4, 6].into()),
                Trait::new("Storyweaver", [3, 5, 7, 10].into()),
                Trait::new("Reaper", [2, 4].into()),
                Trait::new("Sage", [2, 3, 4, 5].into()),
                Trait::new("Sniper", [2, 4, 6].into()),
                Trait::new("Trickshot", [2, 4].into()),
                Trait::new("Umbral", [2, 4, 6, 9].into()),
                Trait::new("Warden", [2, 4, 6].into()),
            ]
                .iter()
                .map(|t| (t.name.clone(), t.clone()))
        );

        #[rustfmt::skip]
        let champions = HashMap::from_iter([
            // 1 costs
            Champion::new(1, "Ahri", ["Fated", "Arcanist"].into(), 4, true),
            Champion::new(1, "Caitlyn", ["Ghostly", "Sniper"].into(), 4, false),
            Champion::new(1, "Cho'Gath", ["Mythic", "Behemoth"].into(), 1, true),
            Champion::new(1, "Darius", ["Umbral", "Duelist"].into(), 1, true),
            Champion::new(1, "Kobuko", ["Fortune", "Bruiser"].into(), 1, true),
            Champion::new(1, "Garen", ["Storyweaver", "Warden"].into(), 1, false),
            Champion::new(1, "Jax", ["Inkshadow", "Warden"].into(), 1, false),
            Champion::new(1, "Kha'Zix", ["Heavenly", "Reaper"].into(), 1, false),
            Champion::new(1, "Kog'Maw", ["Mythic", "Invoker", "Sniper"].into(), 4, false),
            Champion::new(1, "Malphite", ["Heavenly", "Behemoth"].into(), 1, false),
            Champion::new(1, "Rek'Sai", ["Dryad", "Bruiser"].into(), 1, false),
            Champion::new(1, "Sivir", ["Storyweaver", "Trickshot"].into(), 4, false),
            Champion::new(1, "Yasuo", ["Fated", "Duelist"].into(), 1, false),
            
            // 2 costs
            Champion::new(2, "Aatrox", ["Ghostly", "Inkshadow", "Bruiser"].into(), 1, false),
            Champion::new(2, "Gnar", ["Dryad", "Warden"].into(), 1, false),
            Champion::new(2, "Janna", ["Dragonlord", "Invoker"].into(), 4, false),
            Champion::new(2, "Kindred", ["Fated", "Dryad", "Reaper"].into(), 4, false),

            Champion::new(2, "Lux", ["Porcelain", "Arcanist"].into(), 4, false),
            Champion::new(2, "Neeko", ["Mythic", "Heavenly", "Arcanist"].into(), 1, false),
            Champion::new(2, "Qiyana", ["Heavenly", "Duelist"].into(), 1, false),
            Champion::new(2, "Riven", ["Storyweaver", "Altruist", "Bruiser"].into(), 1, false),

            Champion::new(2, "Senna", ["Inkshadow", "Sniper"].into(), 1, false),
            Champion::new(2, "Shen", ["Ghostly", "Behemoth"].into(), 1, false),
            Champion::new(2, "Teemo", ["Fortune", "Trickshot"].into(), 4, false),
            Champion::new(2, "Yorick", ["Umbral", "Behemoth"].into(), 1, false),

            Champion::new(2, "Zyra", ["Storyweaver", "Sage"].into(), 4, false),

            // 3 costs
            Champion::new(3, "Alune", ["Umbral", "Invoker"].into(), 1, false),
            Champion::new(3, "Amumu", ["Porcelain", "Warden"].into(), 1, false),
            Champion::new(3, "Aphelios", ["Fated", "Sniper"].into(), 4, false),
            Champion::new(3, "Bard", ["Mythic", "Trickshot"].into(), 4, false),
            Champion::new(3, "Diana", ["Dragonlord", "Sage"].into(), 1, false),
            Champion::new(3, "Illaoi", ["Ghostly", "Arcanist", "Warden"].into(), 1, false),
            Champion::new(3, "Soraka", ["Heavenly", "Altruist"].into(), 4, false),
            Champion::new(3, "Tahm Kench", ["Mythic", "Bruiser"].into(), 1, false),
            Champion::new(3, "Thresh", ["Fated", "Behemoth"].into(), 2, false),
            Champion::new(3, "Tristana", ["Fortune", "Duelist"].into(), 4, false),
            Champion::new(3, "Volibear", ["Inkshadow", "Duelist"].into(), 1, false),
            Champion::new(3, "Yone", ["Umbral", "Reaper"].into(), 1, false),
            Champion::new(3, "Zoe", ["Fortune", "Storyweaver", "Arcanist"].into(), 4, false),

            // 4 costs
            Champion::new(4, "Annie", ["Fortune", "Invoker"].into(), 1, false),
            Champion::new(4, "Ashe", ["Porcelain", "Sniper"].into(), 4, false),
            Champion::new(4, "Galio", ["Storyweaver", "Bruiser"].into(), 1, false),
            Champion::new(4, "Kai'Sa", ["Inkshadow", "Trickshot"].into(), 4, false),

            Champion::new(4, "Kayn", ["Ghostly", "Reaper"].into(), 1, false),
            Champion::new(4, "Lee Sin", ["Dragonlord", "Duelist"].into(), 1, false),
            Champion::new(4, "Lillia", ["Mythic", "Invoker"].into(), 4, false),
            Champion::new(4, "Morganna", ["Ghostly", "Sage"].into(), 4, false),

            Champion::new(4, "Nautilus", ["Mythic", "Warden"].into(), 1, false),
            Champion::new(4, "Ornn", ["Dryad", "Behemoth"].into(), 1, false),
            Champion::new(4, "Sylas", ["Umbral", "Bruiser"].into(), 1, false),
            Champion::new(4, "Syndra", ["Fated", "Arcanist"].into(), 4, false),

            // 5 costs
            Champion::new(5, "Azir", ["Dryad", "Invoker"].into(), 4, false),
            Champion::new(5, "Hwei", ["Mythic"].into(), 4, false),
            Champion::new(5, "Irelia", ["Storyweaver", "Duelist"].into(), 4, false),
            Champion::new(5, "Lissandra", ["Porcelain", "Arcanist"].into(), 2, false),

            Champion::new(5, "Rakan", ["Dragonlord", "Altruist"].into(), 4, false),
            Champion::new(5, "Sett", ["Fated", "Umbral", "Warden"].into(), 1, false),
            Champion::new(5, "Udyr", ["Inkshadow", "Behemoth"].into(), 1, false),
            Champion::new(5, "Wukong", ["Heavenly", "Sage"].into(), 1, false),

            Champion::new(5, "Xayah", ["Dragonlord", "Trickshot"].into(), 4, false),
        ]
        .iter()
        .map(|ch| (ch.name.clone(), ch.clone())));

        let mut champions_by_trait = HashMap::new();
        for ch in champions.values() {
            for t in ch.traits.iter() {
                let bin = champions_by_trait
                    .entry(t.clone())
                    .or_insert_with(|| Vec::new());
                bin.push(ch.name.clone());
            }
        }

        Self { traits, champions, champions_by_trait }
    }
}
