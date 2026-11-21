// data.rs: the option schema shared with the web GUI.
//
// tools/webgui-data.js dumps homSchema.js/merSchema.js (names, categories, types, enums, defaults,
// min/max and the English descriptions) to data/options.json, which is committed and checked for
// drift in CI. Replicating the schema by hand here would be a second source of truth.

use std::collections::BTreeMap;

use serde_json::Value;

#[derive(Debug, Clone)]
pub struct OptionDef {
    pub name: String,
    pub category: String,
    pub kind: String, // int | bool | enum | double | serverType
    pub default: Value,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub enum_name: Option<String>,
    pub description: String,
}

#[derive(Debug)]
pub struct Schema {
    pub enums: BTreeMap<String, Vec<(String, i64)>>,
    pub options: Vec<OptionDef>,
    pub by_name: BTreeMap<String, usize>,
    pub categories: Vec<String>,
}

impl Schema {
    pub fn get(&self, name: &str) -> Option<&OptionDef> {
        self.by_name.get(name).map(|&i| &self.options[i])
    }

    /// The token the shipped configuration uses for this option's default.
    pub fn default_token(&self, option: &OptionDef) -> String {
        match option.kind.as_str() {
            "bool" => if option.default.as_bool().unwrap_or(false) { "1" } else { "0" }.to_string(),
            "serverType" => if option.default.as_i64().unwrap_or(0) == 1 { "SERVER_PRIVATE" } else { "SERVER_OFFICIAL" }.to_string(),
            "double" => option.default.as_f64().map(|v| format!("{v}")).unwrap_or_else(|| "0".into()),
            _ => option.default.as_i64().map(|v| v.to_string()).unwrap_or_else(|| "0".into()),
        }
    }

    pub fn enum_values(&self, option: &OptionDef) -> Option<&Vec<(String, i64)>> {
        option.enum_name.as_deref().and_then(|name| self.enums.get(name))
    }
}

fn number(value: &Value) -> Option<f64> {
    value.as_f64().or_else(|| value.as_str().and_then(|s| s.parse().ok()))
}

fn parse_options(schema: &Value) -> Result<Vec<OptionDef>, String> {
    let list = schema.as_array().ok_or("schema is not an array")?;
    let mut out = Vec::with_capacity(list.len());
    for entry in list {
        let name = entry.get("name").and_then(|v| v.as_str()).ok_or("option without name")?;
        out.push(OptionDef {
            name: name.to_string(),
            category: entry.get("category").and_then(|v| v.as_str()).unwrap_or("Other").to_string(),
            kind: entry.get("type").and_then(|v| v.as_str()).unwrap_or("int").to_string(),
            default: entry.get("default").cloned().unwrap_or(Value::from(0)),
            min: entry.get("min").and_then(number),
            max: entry.get("max").and_then(number),
            enum_name: entry.get("enum").and_then(|v| v.as_str()).map(|s| s.to_string()),
            description: entry.get("description").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        });
    }
    Ok(out)
}

fn parse_enums(enums: &Value) -> Result<BTreeMap<String, Vec<(String, i64)>>, String> {
    let map = enums.as_object().ok_or("enums is not an object")?;
    let mut out = BTreeMap::new();
    for (name, values) in map {
        let list = values.as_array().ok_or_else(|| format!("{name}: not an array"))?;
        let mut pairs = Vec::with_capacity(list.len());
        for item in list {
            let pair = item.as_array().ok_or_else(|| format!("{name}: entry is not a pair"))?;
            let label = pair.first().and_then(|v| v.as_str()).ok_or_else(|| format!("{name}: label"))?;
            let value = pair.get(1).and_then(|v| v.as_i64()).ok_or_else(|| format!("{name}: value"))?;
            pairs.push((label.to_string(), value));
        }
        out.insert(name.clone(), pairs);
    }
    Ok(out)
}

fn build(enums: &Value, schema: &Value) -> Result<Schema, String> {
    let options = parse_options(schema)?;
    let mut by_name = BTreeMap::new();
    for (index, option) in options.iter().enumerate() {
        by_name.insert(option.name.clone(), index);
    }
    let mut categories: Vec<String> = Vec::new();
    for option in &options {
        if !categories.contains(&option.category) {
            categories.push(option.category.clone());
        }
    }
    Ok(Schema { enums: parse_enums(enums)?, options, by_name, categories })
}

pub fn load() -> Result<(Schema, Schema), String> {
    let text = include_str!("../../../webgui/data/options.json");
    let doc: Value = serde_json::from_str(text).map_err(|e| format!("options.json: {e}"))?;
    let hom = doc.get("hom").ok_or("options.json: no hom")?;
    let mer = doc.get("mer").ok_or("options.json: no mer")?;
    let hom = build(hom.get("enums").unwrap_or(&Value::Null), hom.get("schema").unwrap_or(&Value::Null))
        .map_err(|e| format!("homunculus schema: {e}"))?;
    let mer = build(mer.get("enums").unwrap_or(&Value::Null), mer.get("schema").unwrap_or(&Value::Null))
        .map_err(|e| format!("mercenary schema: {e}"))?;
    Ok((hom, mer))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_the_web_gui_schema() {
        let (hom, mer) = load().unwrap();
        assert_eq!(hom.options.len(), 188);
        assert_eq!(mer.options.len(), 78);
        let aggro = hom.get("AggroHP").expect("AggroHP");
        assert_eq!(aggro.category, "Basic Options");
        assert_eq!(aggro.min, Some(0.0));
        assert_eq!(aggro.max, Some(100.0));
        assert!(aggro.description.contains("seek out and attack"));
        assert_eq!(hom.default_token(aggro), "60");

        let lava = hom.get("LavaSlideMode").expect("LavaSlideMode");
        let values = hom.enum_values(lava).expect("enum values");
        assert!(values.iter().any(|(label, value)| label == "Attack" && *value == 0));

        let server = hom.get("ServerType").expect("ServerType");
        assert_eq!(server.kind, "serverType");
        assert_eq!(hom.default_token(server), "SERVER_OFFICIAL");
    }
}
