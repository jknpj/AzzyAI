// skills.rs: the homunculus skill icons and the setting -> skill mapping from webgui/skills.js.
//
// The 50 PNGs in webgui/img/skills are packed into one 20x20 atlas
// (data/skill-icons.rgba) and the metadata is data/skills.json. The mapping is the same
// `match` substring list the web GUI uses with `AzzySkills.forSetting`.

use std::collections::BTreeMap;

use eframe::egui;
use serde_json::Value;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Skill {
    pub id: u32,
    pub name: String,
    pub name_pt: String,
    pub name_es: String,
    pub matches: Vec<String>,
    pub icon: usize,
}

pub struct Skills {
    pub skills: Vec<Skill>,
    pub width: u32,
    pub height: u32,
    pub tile: u32,
    pub columns: u32,
    pub atlas: &'static [u8],
}

impl Skills {
    /// The skill a setting belongs to, like webgui/skills.js `forSetting`.
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
        skills.push(Skill {
            id,
            name: string(entry, "name"),
            name_pt: string(entry, "name_pt"),
            name_es: string(entry, "name_es"),
            matches,
            icon: entry.get("icon").and_then(Value::as_u64).unwrap_or(0) as usize,
        });
    }
    Ok(Skills { skills, width, height, tile, columns, atlas })
}

#[allow(dead_code)]
pub fn by_id(skills: &[Skill]) -> BTreeMap<u32, usize> {
    skills.iter().enumerate().map(|(index, skill)| (skill.id, index)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_the_skill_atlas_and_mapping() {
        let skills = load().unwrap();
        assert_eq!(skills.skills.len(), 50);
        assert_eq!(skills.atlas.len(), 160 * 140 * 4);
        let lava = skills.for_setting("UseDieterLavaSlide").expect("Lava Slide");
        assert_eq!(lava.id, 8041);
        assert_eq!(lava.name, "Lava Slide");
        assert!(skills.for_setting("AggroHP").is_none());
        let uv = skills.icon_uv(lava.icon);
        assert!(uv.min.x >= 0.0 && uv.max.x <= 1.0);
        assert!(uv.min.y >= 0.0 && uv.max.y <= 1.0);
    }
}
