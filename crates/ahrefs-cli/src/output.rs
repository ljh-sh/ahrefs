use serde_json::{Map, Value};

use crate::cli::Format;

/// What a command returned.
pub enum Payload {
    /// A decoded JSON response.
    Json(Value),
    /// The response body, unchanged.
    Raw(Vec<u8>),
}

/// Renders a payload in the requested format.
pub fn render(format: Format, payload: &Payload) -> String {
    match (format, payload) {
        (Format::Raw, Payload::Raw(body)) => String::from_utf8_lossy(body).into_owned(),
        (Format::Raw, Payload::Json(value)) => value.to_string(),
        (Format::Table, Payload::Json(value)) => table(value),
        (_, Payload::Json(value)) => {
            let mut envelope = Map::new();
            envelope.insert("ok".into(), Value::Bool(true));
            envelope.insert("data".into(), value.clone());
            Value::Object(envelope).to_string()
        }
        (_, Payload::Raw(body)) => String::from_utf8_lossy(body).into_owned(),
    }
}

/// Renders an error as a JSON object for the error stream.
pub fn error(kind: &str, message: &str, status: Option<u16>) -> String {
    let mut detail = Map::new();
    detail.insert("kind".into(), Value::String(kind.to_string()));
    detail.insert("message".into(), Value::String(message.to_string()));
    if let Some(status) = status {
        detail.insert("status".into(), Value::from(status));
    }

    let mut envelope = Map::new();
    envelope.insert("ok".into(), Value::Bool(false));
    envelope.insert("error".into(), Value::Object(detail));
    Value::Object(envelope).to_string()
}

/// Renders a JSON response as a table.
pub fn table(value: &Value) -> String {
    match value {
        Value::Object(map) => {
            if let Some(Value::Array(rows)) = map.values().find(|v| v.is_array()) {
                return rows_table(rows);
            }
            if map.len() == 1 {
                if let Some(Value::Object(inner)) = map.values().next() {
                    return object_table(inner);
                }
            }
            object_table(map)
        }
        Value::Array(rows) => rows_table(rows),
        other => other.to_string(),
    }
}

/// Renders a list of objects as a table.
fn rows_table(rows: &[Value]) -> String {
    let mut columns: Vec<String> = Vec::new();
    for row in rows {
        if let Some(map) = row.as_object() {
            for key in map.keys() {
                if !columns.contains(key) {
                    columns.push(key.clone());
                }
            }
        }
    }
    if columns.is_empty() {
        return "(no rows)".to_string();
    }

    let cells: Vec<Vec<String>> = rows
        .iter()
        .map(|row| {
            columns
                .iter()
                .map(|column| match row.get(column) {
                    Some(Value::Null) | None => String::new(),
                    Some(Value::String(s)) => s.clone(),
                    Some(other) => other.to_string(),
                })
                .collect()
        })
        .collect();

    let mut widths: Vec<usize> = columns.iter().map(|c| c.chars().count()).collect();
    for row in &cells {
        for (index, cell) in row.iter().enumerate() {
            widths[index] = widths[index].max(cell.chars().count());
        }
    }

    let mut out = String::new();
    push_row(&mut out, &columns, &widths);
    for row in &cells {
        push_row(&mut out, row, &widths);
    }
    out.trim_end().to_string()
}

/// Appends one padded row to a table.
fn push_row(out: &mut String, cells: &[String], widths: &[usize]) {
    for (index, cell) in cells.iter().enumerate() {
        if index > 0 {
            out.push_str("  ");
        }
        out.push_str(cell);
        if index + 1 < cells.len() {
            let pad = widths[index].saturating_sub(cell.chars().count());
            out.push_str(&" ".repeat(pad));
        }
    }
    out.push('\n');
}

/// Renders a single object as a two column table.
fn object_table(map: &Map<String, Value>) -> String {
    if map.is_empty() {
        return "(empty)".to_string();
    }
    let width = map
        .keys()
        .map(|k| k.chars().count())
        .max()
        .unwrap_or_default();

    let mut out = String::new();
    for (key, value) in map {
        let text = match value {
            Value::Null => String::new(),
            Value::String(s) => s.clone(),
            other => other.to_string(),
        };
        out.push_str(key);
        out.push_str(&" ".repeat(width - key.chars().count()));
        out.push_str("  ");
        out.push_str(&text);
        out.push('\n');
    }
    out.trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn json_format_wraps_the_payload() {
        let rendered = render(Format::Json, &Payload::Json(json!({"a": 1})));
        assert_eq!(rendered, r#"{"data":{"a":1},"ok":true}"#);
    }

    #[test]
    fn table_format_lists_rows() {
        let rendered = table(&json!({"rows": [{"a": 1, "b": "x"}, {"a": 2}]}));
        assert_eq!(rendered, "a  b\n1  x\n2");
    }

    #[test]
    fn table_format_falls_back_to_key_values() {
        let rendered = table(&json!({"domain_rating": {"domain_rating": 42.0}}));
        assert_eq!(rendered, "domain_rating  42.0");
    }
}
