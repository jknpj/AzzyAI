// skills.rs: the homunculus skill icons and the setting -> skill mapping from webgui/skills.js.
//
// The 50 PNGs in webgui/img/skills are packed into one 20x20 atlas
// (data/skill-icons.rgba) and the metadata is data/skills.json. The mapping is the same
// `match` substring list the web GUI uses with `AzzySkills.forSetting`.

use eframe::egui;
use serde_json::Value;

use crate::i18n::Lang;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Skill {
    pub id: u32,
    pub homun: String,
    pub name: String,
    pub name_pt: String,
    pub name_es: String,
    pub matches: Vec<String>,
    pub icon: usize,
    /// Official data from webgui/skilldata.js: numbers at max level and the in-game description
    /// (with the client's ^RRGGBB colour codes), in the three languages.
    pub maxlv: u32,
    pub stats: [i64; 6],
    pub desc_en: String,
    pub desc_pt: String,
    pub desc_es: String,
}

pub struct Skills {
    pub skills: Vec<Skill>,
    pub width: u32,
    pub height: u32,
    pub tile: u32,
    pub columns: u32,
    pub atlas: &'static [u8],
}

impl Skill {
    /// Official name in the language, falling back to English (like `AzzySkills.name`).
    pub fn name_for(&self, lang: Lang) -> &str {
        let text = match lang {
            Lang::Pt => &self.name_pt,
            Lang::Es => &self.name_es,
            Lang::En => &self.name,
        };
        if text.is_empty() {
            &self.name
        } else {
            text
        }
    }

    /// In-game description in the language, falling back to English.
    pub fn description_for(&self, lang: Lang) -> &str {
        let text = match lang {
            Lang::Pt => &self.desc_pt,
            Lang::Es => &self.desc_es,
            Lang::En => &self.desc_en,
        };
        if text.is_empty() {
            &self.desc_en
        } else {
            text
        }
    }
}

impl Skills {
    /// The skill a setting belongs to, like webgui/skills.js `forSetting`.
    pub fn get(&self, id: u32) -> Option<&Skill> {
        self.skills.iter().find(|skill| skill.id == id)
    }

    pub fn for_setting(&self, name: &str) -> Option<&Skill> {
        let lower = name.to_lowercase();
        self.skills
            .iter()
            .find(|skill| skill.matches.iter().any(|needle| lower.contains(needle)))
    }

    /// Normalized UV rectangle of one icon inside the atlas.
    pub fn icon_uv(&self, icon: usize) -> egui::Rect {
        let tile = self.tile as f32;
        let column = (icon as u32 % self.columns) as f32;
        let row = (icon as u32 / self.columns) as f32;
        let min = egui::pos2(
            column * tile / self.width as f32,
            row * tile / self.height as f32,
        );
        let max = egui::pos2(
            (column + 1.0) * tile / self.width as f32,
            (row + 1.0) * tile / self.height as f32,
        );
        egui::Rect::from_min_max(min, max)
    }
}

fn string(list: &Value, key: &str) -> String {
    list.get(key).and_then(Value::as_str).unwrap_or("").to_string()
}

pub fn load() -> Result<Skills, String> {
    let text = include_str!("../data/skills.json");
    let doc: Value = serde_json::from_str(text).map_err(|e| format!("skills.json: {e}"))?;
    let detail: Value = serde_json::from_str(include_str!("../../../webgui/data/skilldata.json"))
        .map_err(|e| format!("skilldata.json: {e}"))?;
    let width = doc.get("width").and_then(Value::as_u64).unwrap_or(0) as u32;
    let height = doc.get("height").and_then(Value::as_u64).unwrap_or(0) as u32;
    let tile = doc.get("tile").and_then(Value::as_u64).unwrap_or(0) as u32;
    let columns = doc.get("columns").and_then(Value::as_u64).unwrap_or(1) as u32;
    let atlas: &'static [u8] = include_bytes!("../data/skill-icons.rgba");
    if atlas.len() != (width * height * 4) as usize {
        return Err(format!(
            "skill-icons.rgba: {} bytes, expected {}x{} RGBA",
            atlas.len(),
            width,
            height
        ));
    }
    let list = doc
        .get("skills")
        .and_then(Value::as_array)
        .ok_or("skills.json: no skills")?;
    let mut skills = Vec::with_capacity(list.len());
    for entry in list {
        let id = entry.get("id").and_then(Value::as_u64).unwrap_or(0) as u32;
        let matches = entry
            .get("match")
            .and_then(Value::as_array)
            .map(|list| list.iter().filter_map(Value::as_str).map(str::to_string).collect())
            .unwrap_or_default();
        let official = detail.get(id.to_string()).cloned().unwrap_or(Value::Null);
        let official_name = |key: &str| {
            official
                .pointer(&format!("/name/{key}"))
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string()
        };
        let stats: Vec<i64> = official
            .get("stats")
            .and_then(Value::as_array)
            .map(|list| list.iter().filter_map(Value::as_i64).collect())
            .unwrap_or_default();
        let mut stats6 = [0i64; 6];
        for (slot, value) in stats6.iter_mut().zip(stats) {
            *slot = value;
        }
        let desc = |key: &str| {
            official
                .pointer(&format!("/desc/{key}"))
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string()
        };
        let name = string(entry, "name");
        skills.push(Skill {
            id,
            homun: string(entry, "homun"),
            name: if official_name("en").is_empty() { name } else { official_name("en") },
            name_pt: official_name("pt"),
            name_es: official_name("es"),
            matches,
            icon: entry.get("icon").and_then(Value::as_u64).unwrap_or(0) as usize,
            maxlv: official.get("maxlv").and_then(Value::as_u64).unwrap_or(0) as u32,
            stats: stats6,
            desc_en: desc("en"),
            desc_pt: desc("pt"),
            desc_es: desc("es"),
        });
    }
    Ok(Skills { skills, width, height, tile, columns, atlas })
}

pub const HOMUNCULI: [(&str, &str); 4] = [
    ("lif", "Lif"),
    ("amistr", "Amistr"),
    ("filir", "Filir"),
    ("vanilmirth", "Vanilmirth"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_the_skill_atlas_and_mapping() {
        let skills = load().unwrap();
        assert_eq!(skills.skills.len(), 13);
        assert_eq!(skills.atlas.len(), 96 * 96 * 4);
        let bulwark = skills.for_setting("AmiBulwarkLevel").expect("Amistr Bulwark");
        assert_eq!(bulwark.id, 8006);
        assert_eq!(bulwark.name, "Amistr Bulwark");
        assert!(skills.for_setting("AggroHP").is_none());
        assert_eq!(bulwark.maxlv, 5);
        assert_eq!(bulwark.stats[0], 20);
        assert_eq!(bulwark.name_for(Lang::Pt), "Fortaleza");
        let uv = skills.icon_uv(bulwark.icon);
        assert!(uv.min.x >= 0.0 && uv.max.x <= 1.0);
        assert!(uv.min.y >= 0.0 && uv.max.y <= 1.0);
    }
}
