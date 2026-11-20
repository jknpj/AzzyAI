// yaml.rs: block-style YAML as used by rAthena's databases - nested mappings and sequences,
// plain/quoted scalars and comments. No anchors, flow collections or multi-line strings; anything
// outside that shape is an error instead of a wrong model. A port of the parser in
// tools/harness/rathena_sync.py.
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Str(String),
    Seq(Vec<Value>),
    Map(BTreeMap<String, Value>),
}

impl Value {
    pub fn get(&self, key: &str) -> Option<&Value> {
        match self {
            Value::Map(map) => map.get(key),
            _ => None,
        }
    }

    pub fn get_int(&self, key: &str) -> Option<i64> {
        self.get(key).and_then(|v| match v {
            Value::Int(i) => Some(*i),
            Value::Str(s) => s.parse().ok(),
            _ => None,
        })
    }

    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.get(key).and_then(|v| v.as_str())
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key).and_then(|v| v.as_bool())
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Str(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_seq(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Seq(list) => Some(list),
            _ => None,
        }
    }
}

struct Line {
    indent: usize,
    text: String,
}

pub fn parse(text: &str) -> Result<Value, String> {
    let mut lines: Vec<Line> = Vec::new();
    for raw in text.lines() {
        let body = raw.trim_end();
        let stripped = body.trim_start_matches(' ');
        let indent = body.len() - stripped.len();
        if !stripped.is_empty() && !stripped.starts_with('#') && stripped != "---" {
            lines.push(Line { indent, text: stripped.to_string() });
        }
    }
    let mut pos = 0usize;
    let value = block(&lines, &mut pos, lines.first().map(|l| l.indent).unwrap_or(0))?;
    if pos != lines.len() {
        return Err(format!("unexpected line: {:?}", lines[pos].text));
    }
    Ok(value)
}

fn block(lines: &[Line], pos: &mut usize, _key_indent: usize) -> Result<Value, String> {
    // like the Python parser: the child's own indentation decides mapping/sequence, not the key's
    let indent = lines[*pos].indent;
    if lines[*pos].text.starts_with('-') {
        return seq(lines, pos, indent);
    }
    mapping(lines, pos, indent, None)
}

fn mapping(lines: &[Line], pos: &mut usize, indent: usize, first: Option<String>) -> Result<Value, String> {
    let mut out = BTreeMap::new();
    let mut first = first;
    while *pos < lines.len() || first.is_some() {
        let (ind, txt) = match first.take() {
            // the rewritten sequence line: its keys line up at the computed indent
            Some(text) => (indent, text),
            None => {
                if *pos >= lines.len() { break; }
                (lines[*pos].indent, lines[*pos].text.clone())
            }
        };
        if ind < indent || (ind == indent && txt.starts_with('-')) {
            break;
        }
        if ind > indent {
            *pos += 1; // continuation of a multi-line scalar; not needed here
            continue;
        }
        let Some((key, value)) = split_key(&txt)? else {
            *pos += 1;
            continue;
        };
        *pos += 1;
        let value = match value {
            Some(v) if !v.trim().is_empty() => scalar(v.trim())?,
            None | Some(_) => {
                let more = *pos < lines.len()
                    && (lines[*pos].indent > ind
                        || (lines[*pos].indent == ind && lines[*pos].text.starts_with('-')));
                if more { block(lines, pos, ind)? } else { Value::Null }
            }
        };
        out.insert(key, value);
    }
    Ok(Value::Map(out))
}

fn seq(lines: &[Line], pos: &mut usize, indent: usize) -> Result<Value, String> {
    let mut out = Vec::new();
    while *pos < lines.len() {
        let (ind, ref txt) = (lines[*pos].indent, lines[*pos].text.clone());
        if ind != indent || !txt.starts_with('-') {
            break;
        }
        let rest = txt[1..].trim_start_matches(' ').to_string();
        if rest.is_empty() {
            *pos += 1;
            let more = *pos < lines.len() && lines[*pos].indent > indent;
            out.push(if more { block(lines, pos, indent + 1)? } else { Value::Null });
        } else if rest.contains(": ") || rest.ends_with(':') {
            if rest.starts_with('"') || rest.starts_with('\'') {
                *pos += 1;
                out.push(scalar(&rest)?);
            } else {
                // "- Key: value" starts a mapping whose keys line up after the dash: the line is
                // rewritten without the dash (same indent math as the Python parser) and the
                // mapping consumes the ORIGINAL list from there, so nested sequences and the
                // following entries are handled by their own indentation levels
                let inner_indent = ind + 1 + (txt[1..].len() - txt[1..].trim_start_matches(' ').len());
                out.push(mapping(lines, pos, inner_indent, Some(rest))?);
            }
        } else {
            *pos += 1;
            out.push(scalar(&rest)?);
        }
    }
    Ok(Value::Seq(out))
}

/// "Key: value" -> (key, Some(value)) / "Key:" -> (key, None). None means the key is not a
/// mapping key at all.
fn split_key(txt: &str) -> Result<Option<(String, Option<String>)>, String> {
    // find the first ':' that is not inside quotes
    let bytes = txt.as_bytes();
    let mut quote: Option<u8> = None;
    for (i, &b) in bytes.iter().enumerate() {
        match quote {
            Some(q) if b == q => quote = None,
            None if b == b'"' || b == b'\'' => quote = Some(b),
            None if b == b':' => {
                let after = txt[i + 1..].chars().next();
                if after.is_none() || after == Some(' ') {
                    let key = txt[..i].trim().to_string();
                    if key.contains(' ') && !key.starts_with('"') && !key.starts_with('\'') {
                        // a plain scalar, not a key
                        return Ok(None);
                    }
                    let value = txt[i + 1..].trim().to_string();
                    return Ok(Some((unquote_key(&key)?, if value.is_empty() { None } else { Some(value) })));
                }
            }
            _ => {}
        }
    }
    Ok(None)
}

fn unquote_key(key: &str) -> Result<String, String> {
    if key.starts_with('"') && key.ends_with('"') && key.len() >= 2 {
        return Ok(key[1..key.len() - 1].replace("\\\"", "\"").replace("\\\\", "\\"));
    }
    if key.starts_with('\'') && key.ends_with('\'') && key.len() >= 2 {
        return Ok(key[1..key.len() - 1].replace("''", "'"));
    }
    Ok(key.to_string())
}

fn scalar(v: &str) -> Result<Value, String> {
    if let Some(stripped) = v.strip_prefix('"') {
        let end = stripped.rfind('"').ok_or_else(|| format!("unterminated string: {v:?}"))?;
        return Ok(Value::Str(stripped[..end].replace("\\\"", "\"").replace("\\\\", "\\")));
    }
    if let Some(stripped) = v.strip_prefix('\'') {
        let end = stripped.rfind('\'').ok_or_else(|| format!("unterminated string: {v:?}"))?;
        return Ok(Value::Str(stripped[..end].replace("''", "'")));
    }
    let v = v.split('#').next().unwrap_or(v).trim_end();
    match v {
        "true" => Ok(Value::Bool(true)),
        "false" => Ok(Value::Bool(false)),
        "null" | "~" | "" => Ok(Value::Null),
        _ => {
            // integers only; codes with leading zeros (Ai: 04) stay strings
            if let Ok(i) = v.parse::<i64>() {
                if !(v.len() > 1 && v.starts_with('0')) {
                    return Ok(Value::Int(i));
                }
            }
            Ok(Value::Str(v.to_string()))
        }
    }
}
