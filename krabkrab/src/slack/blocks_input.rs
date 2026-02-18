use serde_json::Value;

const SLACK_MAX_BLOCKS: usize = 50;

fn assert_blocks_array(raw: &Value) -> Result<(), String> {
    if !raw.is_array() {
        return Err("blocks must be an array".to_string());
    }
    let arr = raw.as_array().unwrap();
    if arr.is_empty() {
        return Err("blocks must contain at least one block".to_string());
    }
    if arr.len() > SLACK_MAX_BLOCKS {
        return Err(format!("blocks cannot exceed {} items", SLACK_MAX_BLOCKS));
    }
    for block in arr {
        if block.is_null() || !block.is_object() {
            return Err("each block must be an object".to_string());
        }
        let typ = block.get("type");
        match typ {
            Some(Value::String(s)) if !s.trim().is_empty() => {}
            _ => return Err("each block must include a non-empty string type".to_string()),
        }
    }
    Ok(())
}

pub fn validate_slack_blocks_array(raw: &Value) -> Result<Vec<Value>, String> {
    assert_blocks_array(raw)?;
    Ok(raw.as_array().unwrap().clone())
}

pub fn parse_slack_blocks_input(raw: &Value) -> Result<Option<Vec<Value>>, String> {
    if raw.is_null() {
        return Ok(None);
    }
    let parsed = if raw.is_string() {
        let s = raw.as_str().unwrap();
        serde_json::from_str::<Value>(s).map_err(|_| "blocks must be valid JSON".to_string())?
    } else {
        raw.clone()
    };
    Ok(Some(validate_slack_blocks_array(&parsed)?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn accepts_valid_array() {
        let v = json!([ {"type": "section", "text": {"type": "mrkdwn", "text": "hi"}} ]);
        let out = validate_slack_blocks_array(&v).unwrap();
        assert_eq!(out.len(), 1);
    }

    #[test]
    fn rejects_non_array() {
        let v = json!({"type":"section"});
        assert!(validate_slack_blocks_array(&v).is_err());
    }

    #[test]
    fn rejects_empty_array() {
        let v = json!([]);
        assert!(validate_slack_blocks_array(&v).is_err());
    }

    #[test]
    fn rejects_missing_type() {
        let v = json!([ {"text": "no type"} ]);
        assert!(validate_slack_blocks_array(&v).is_err());
    }

    #[test]
    fn parse_string_json() {
        let s = "[{\"type\":\"section\"}]";
        let v = Value::String(s.to_string());
        let out = parse_slack_blocks_input(&v).unwrap();
        assert!(out.is_some());
    }
}
