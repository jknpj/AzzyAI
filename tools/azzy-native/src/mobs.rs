// mobs.rs: monster and map data from webgui/mobdata.js, dumped to data/mobs.json by
// tools/webgui-data.js (checked in CI, one source of truth with the web GUI).
//
// The tactics editor uses the names for rows that have no `--Name` comment and to tell you what
// an id means while you type it.

use std::collections::BTreeMap;

use serde_json::Value;

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
}

impl Mobs {
    pub fn get(&self, id: &str) -> Option<&Mob> {
        id.trim().parse::<u32>().ok().and_then(|id| self.by_id.get(&id))
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
}

fn strings(value: Option<&Value>) -> Vec<String> {
    value
        .and_then(Value::as_array)
        .map(|list| list.iter().filter_map(Value::as_str).map(str::to_string).collect())
        .unwrap_or_default()
}

pub fn load() -> Result<Mobs, String> {
    let text = include_str!("../data/mobs.json");
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
    Ok(Mobs { elements, races, by_id })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_the_monster_database() {
        let mobs = load().unwrap();
        assert_eq!(mobs.by_id.len(), 2675);
        let poring = mobs.get("1002").expect("Poring");
        assert_eq!(poring.name, "Poring");
        assert_eq!(poring.level, 1);
        assert_eq!(mobs.element_name(poring), "Water");
        assert_eq!(mobs.race_name(poring), "Plant");
        assert!(mobs.get("not-a-number").is_none());
    }
}
