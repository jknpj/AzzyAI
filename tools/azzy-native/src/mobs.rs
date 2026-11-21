// mobs.rs: monster and map data from webgui/mobdata.js, dumped to data/mobs.json by
// tools/webgui-data.js (checked in CI, one source of truth with the web GUI).
//
// The tactics editor uses the names for rows that have no `--Name` comment, tells you what an id
// means while you type it, and builds the "suggested tactics" rows from the same flags the web
// GUI uses in webgui/tactSuggest.js.

use std::collections::BTreeMap;

use serde_json::Value;

// mobdata.js flags (sum): 1 boss, 2 MVP, 4 plant, 8 aggressive, 16 can't move.
pub const BOSS: i64 = 1;
pub const MVP: i64 = 2;
pub const PLANT: i64 = 4;
pub const AGGRESSIVE: i64 = 8;
pub const IMMOBILE: i64 = 16;

#[derive(Debug, Clone)]
pub struct Mob {
    pub id: u32,
    pub name: String,
    pub level: i64,
    pub flags: i64,
    pub element: u8,
    pub element_level: u8,
    pub race: u8,
}

pub struct Mobs {
    pub elements: Vec<String>,
    pub races: Vec<String>,
    pub by_id: BTreeMap<u32, Mob>,
    pub maps: BTreeMap<String, Vec<u32>>,
}

#[derive(Debug, Clone)]
pub struct Suggested {
    pub id: String,
    pub name: String,
    pub values: Vec<String>,
}

impl Mobs {
    pub fn get(&self, id: &str) -> Option<&Mob> {
        id.trim().parse::<u32>().ok().and_then(|id| self.by_id.get(&id))
    }

    pub fn mob(&self, id: u32) -> Option<&Mob> {
        self.by_id.get(&id)
    }

    pub fn name(&self, id: &str) -> Option<&str> {
        self.get(id).map(|mob| mob.name.as_str())
    }

    pub fn element_name(&self, mob: &Mob) -> &str {
        self.elements.get(mob.element as usize).map(String::as_str).unwrap_or("?")
    }

    pub fn race_name(&self, mob: &Mob) -> &str {
        self.races.get(mob.race as usize).map(String::as_str).unwrap_or("?")
    }

    /// "Poring (Water 1)", the row name suggested by the web GUI (same monster names can differ
    /// by element).
    pub fn row_name(&self, mob: &Mob) -> String {
        format!("{} ({} {})", mob.name, self.element_name(mob), mob.element_level)
    }

    /// "Poring #1002 - Lv 1, Water 1", the search label from webgui/tactSuggest.js.
    pub fn label(&self, mob: &Mob) -> String {
        format!(
            "{} #{} - Lv {}, {} {}",
            mob.name,
            mob.id,
            mob.level,
            self.element_name(mob),
            mob.element_level
        )
    }

    /// The first 8 monsters whose name (or id) matches `query`.
    pub fn search(&self, query: &str, limit: usize) -> Vec<&Mob> {
        let needle = query.trim().to_lowercase();
        if needle.is_empty() {
            return Vec::new();
        }
        let mut matches: Vec<&Mob> = self
            .by_id
            .values()
            .filter(|mob| {
                mob.name.to_lowercase().contains(&needle) || mob.id.to_string().contains(&needle)
            })
            .collect();
        // prefix matches first ("Poring" before "Poporing"), then the rest by name
        matches.sort_by_key(|mob| {
            let name = mob.name.to_lowercase();
            (if name.starts_with(&needle) { 0 } else { 1 }, name, mob.id)
        });
        matches.truncate(limit);
        matches
    }

    pub fn map_names(&self) -> Vec<&str> {
        self.maps.keys().map(String::as_str).collect()
    }

    pub fn map_mobs(&self, map: &str) -> Option<&Vec<u32>> {
        self.maps.get(map.trim())
    }

    pub fn plants(&self) -> Vec<u32> {
        self.by_id
            .values()
            .filter(|mob| mob.flags & PLANT != 0)
            .map(|mob| mob.id)
            .collect()
    }

    /// "Dimik #1672 - ..." or "1672" -> 1672, like tactSuggest.js.
    pub fn parse_mob(text: &str) -> Option<u32> {
        let text = text.trim();
        if let Some(hash) = text.find('#') {
            let digits: String = text[hash + 1..]
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect();
            if let Ok(id) = digits.parse() {
                return Some(id);
            }
        }
        text.parse().ok()
    }

    /// The tactics row for a monster id, or None when the database does not know it.
    pub fn suggest(&self, id: u32) -> Option<Suggested> {
        let mob = self.by_id.get(&id)?;
        let flags = mob.flags;
        let values = if flags & PLANT != 0 {
            row("TACT_REACT_L", "SKILL_NEVER", "0", None, None)
        } else if flags & MVP != 0 {
            row("TACT_REACT_H", "SKILL_ALWAYS", "1", None, None)
        } else if flags & BOSS != 0 {
            row("TACT_REACT_M", "SKILL_ALWAYS", "1", None, None)
        } else if flags & AGGRESSIVE != 0 && flags & IMMOBILE != 0 {
            row("TACT_IGNORE", "SKILL_NEVER", "1", Some("CHASE_NEVER"), Some("SNIPE_DISABLE"))
        } else if flags & IMMOBILE != 0 {
            row("TACT_ATTACK_L", "SKILL_NEVER", "1", None, None)
        } else if flags & AGGRESSIVE != 0 {
            row("TACT_ATTACK_H", "SKILL_ALWAYS", "1", None, None)
        } else {
            row("TACT_ATTACK_M", "SKILL_ALWAYS", "1", None, None)
        };
        Some(Suggested {
            id: mob.id.to_string(),
            name: self.row_name(mob),
            values,
        })
    }

    pub fn rules() -> &'static [&'static str] {
        &[
            "Plants and mushrooms: only when they are hit, no skills, no weight",
            "MVPs: only when they attack, all skills",
            "Other bosses: only when they attack, all skills",
            "Aggressive monsters that can't move: ignored, never chased",
            "Harmless monsters that can't move (eggs): last, no skills",
            "Aggressive monsters: attacked first, all skills",
            "Passive monsters: attacked after the aggressive ones, all skills",
        ]
    }
}

fn row(
    basic: &str,
    skill: &str,
    weight: &str,
    chase: Option<&str>,
    snipe: Option<&str>,
) -> Vec<String> {
    vec![
        basic.to_string(),
        skill.to_string(),
        "KITE_NEVER".to_string(),
        "CAST_REACT".to_string(),
        "PUSH_NEVER".to_string(),
        "DEBUFF_NEVER".to_string(),
        "CLASS_BOTH".to_string(),
        "RESCUE_NEVER".to_string(),
        "-1".to_string(),
        snipe.unwrap_or("SNIPE_OK").to_string(),
        "KS_NEVER".to_string(),
        weight.to_string(),
        chase.unwrap_or("CHASE_NORMAL").to_string(),
    ]
}

fn strings(value: Option<&Value>) -> Vec<String> {
    value
        .and_then(Value::as_array)
        .map(|list| list.iter().filter_map(Value::as_str).map(str::to_string).collect())
        .unwrap_or_default()
}

pub fn load() -> Result<Mobs, String> {
    let text = include_str!("../../../webgui/data/mobs.json");
    let doc: Value = serde_json::from_str(text).map_err(|e| format!("mobs.json: {e}"))?;
    let elements = strings(doc.get("elements"));
    let races = strings(doc.get("races"));
    let mut by_id = BTreeMap::new();
    let mobs = doc.get("mobs").and_then(Value::as_object).ok_or("mobs.json: no mobs")?;
    for (key, value) in mobs {
        let id: u32 = key.parse().map_err(|_| format!("mobs.json: bad id {key}"))?;
        let list = value.as_array().ok_or_else(|| format!("mobs.json: mob {key} is not an array"))?;
        let name = list.first().and_then(Value::as_str).unwrap_or("").to_string();
        let number = |index: usize| list.get(index).and_then(Value::as_i64).unwrap_or(0);
        by_id.insert(id, Mob {
            id,
            name,
            level: number(1),
            flags: number(2),
            element: number(3).clamp(0, 255) as u8,
            element_level: number(4).clamp(0, 255) as u8,
            race: number(5).clamp(0, 255) as u8,
        });
    }
    let mut maps = BTreeMap::new();
    if let Some(list) = doc.get("maps").and_then(Value::as_object) {
        for (name, value) in list {
            let ids = value
                .as_array()
                .map(|ids| {
                    ids.iter()
                        .filter_map(Value::as_i64)
                        .filter_map(|id| u32::try_from(id).ok())
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            maps.insert(name.clone(), ids);
        }
    }
    Ok(Mobs { elements, races, by_id, maps })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_the_monster_database() {
        let mobs = load().unwrap();
        assert_eq!(mobs.by_id.len(), 1004);
        let poring = mobs.get("1002").expect("Poring");
        assert_eq!(poring.name, "Poring");
        assert_eq!(poring.level, 1);
        assert_eq!(mobs.element_name(poring), "Water");
        assert_eq!(mobs.race_name(poring), "Plant");
        assert!(mobs.get("not-a-number").is_none());
        assert!(mobs.maps.len() > 300, "maps: {}", mobs.maps.len());
        assert!(mobs.map_mobs("prt_fild08").is_some());
    }

    #[test]
    fn suggests_rows_from_the_flags() {
        let mobs = load().unwrap();
        let poring = mobs.suggest(1002).expect("Poring");
        assert_eq!(poring.name, "Poring (Water 1)");
        assert_eq!(poring.values.len(), 13);
        assert_eq!(poring.values[0], "TACT_ATTACK_M");
        assert_eq!(poring.values[1], "SKILL_ALWAYS");

        let plant = mobs.suggest(1078).expect("Red Plant");
        assert_eq!(plant.values[0], "TACT_REACT_L");
        assert_eq!(plant.values[1], "SKILL_NEVER");
        assert_eq!(plant.values[11], "0");

        assert!(mobs.suggest(999999).is_none());
    }

    #[test]
    fn parses_a_monster_search_entry() {
        assert_eq!(Mobs::parse_mob("Dimik #1672 - Lv 116, Earth 2"), Some(1672));
        assert_eq!(Mobs::parse_mob(" 1672 "), Some(1672));
        assert_eq!(Mobs::parse_mob("Poring"), None);
    }

    #[test]
    fn searches_by_name_and_id() {
        let mobs = load().unwrap();
        let matches = mobs.search("poring", 8);
        assert!(matches.iter().any(|mob| mob.id == 1002));
        assert!(matches.len() <= 8);
        assert!(mobs.search("", 8).is_empty());
    }
}
