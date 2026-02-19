use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Core message and user types
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct UserId(pub String);

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Message {
    pub id: String,
    pub text: String,
    pub from: Option<UserId>,
}

impl Message {
    pub fn simple(text: &str) -> Self {
        Self {
            id: "msg-1".into(),
            text: text.to_string(),
            from: None,
        }
    }
}

/// Tool input error for parameter validation
#[derive(Debug, thiserror::Error)]
#[error("Tool input error: {message}")]
pub struct ToolInputError {
    pub message: String,
}

impl ToolInputError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

/// Parameter reading options
#[derive(Debug, Clone, Default)]
pub struct StringParamOptions {
    pub required: bool,
    pub trim: bool,
    pub label: Option<String>,
    pub allow_empty: bool,
}

#[derive(Debug, Clone, Default)]
pub struct BooleanParamOptions {
    pub required: bool,
    pub default_value: Option<bool>,
}

#[derive(Debug, Clone, Default)]
pub struct NumberParamOptions {
    pub required: bool,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub default_value: Option<f64>,
}

#[derive(Debug, Clone, Default)]
pub struct PathParamOptions {
    pub required: bool,
    pub must_exist: bool,
    pub must_be_file: bool,
    pub must_be_dir: bool,
    pub allow_relative: bool,
}

/// Action gate for controlling tool behavior
pub type ActionGate<T> = dyn Fn(&str, Option<bool>) -> bool + Send + Sync;

/// Create an action gate from a configuration map
pub fn create_action_gate<T: std::any::Any>(
    actions: Option<HashMap<String, bool>>,
) -> Box<ActionGate<T>> {
    let actions = actions.unwrap_or_default();
    Box::new(move |key: &str, default_value: Option<bool>| -> bool {
        actions.get(key).copied().unwrap_or(default_value.unwrap_or(true))
    })
}

/// Read and validate a string parameter
pub fn read_string_param(
    params: &HashMap<String, serde_json::Value>,
    key: &str,
    options: StringParamOptions,
) -> Result<String, ToolInputError> {
    let value = params.get(key);

    if value.is_none() {
        if options.required {
            return Err(ToolInputError::new(format!(
                "Parameter '{}' is required{}",
                key,
                options.label.as_ref().map(|l| format!(" ({})", l)).unwrap_or_default()
            )));
        }
        return Ok(String::new());
    }

    let value = value.unwrap();
    let string_value = match value {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        _ => {
            return Err(ToolInputError::new(format!(
                "Parameter '{}' must be a string{}",
                key,
                options.label.as_ref().map(|l| format!(" ({})", l)).unwrap_or_default()
            )));
        }
    };

    let mut result = if options.trim {
        string_value.trim().to_string()
    } else {
        string_value
    };

    if !options.allow_empty && result.is_empty() {
        return Err(ToolInputError::new(format!(
            "Parameter '{}' cannot be empty{}",
            key,
            options.label.as_ref().map(|l| format!(" ({})", l)).unwrap_or_default()
        )));
    }

    Ok(result)
}

/// Read and validate a boolean parameter
pub fn read_boolean_param(
    params: &HashMap<String, serde_json::Value>,
    key: &str,
    options: BooleanParamOptions,
) -> Result<bool, ToolInputError> {
    let value = params.get(key);

    if value.is_none() {
        if options.required {
            return Err(ToolInputError::new(format!("Parameter '{}' is required", key)));
        }
        return Ok(options.default_value.unwrap_or(false));
    }

    let value = value.unwrap();
    match value {
        serde_json::Value::Bool(b) => Ok(*b),
        serde_json::Value::String(s) => match s.to_lowercase().as_str() {
            "true" | "1" | "yes" | "on" => Ok(true),
            "false" | "0" | "no" | "off" => Ok(false),
            _ => Err(ToolInputError::new(format!("Parameter '{}' must be a valid boolean", key))),
        },
        serde_json::Value::Number(n) => Ok(*n.as_i64().unwrap_or(0) != 0),
        _ => Err(ToolInputError::new(format!("Parameter '{}' must be a boolean", key))),
    }
}

/// Read and validate a number parameter
pub fn read_number_param(
    params: &HashMap<String, serde_json::Value>,
    key: &str,
    options: NumberParamOptions,
) -> Result<f64, ToolInputError> {
    let value = params.get(key);

    if value.is_none() {
        if options.required {
            return Err(ToolInputError::new(format!("Parameter '{}' is required", key)));
        }
        return Ok(options.default_value.unwrap_or(0.0));
    }

    let value = value.unwrap();
    let num_value = match value {
        serde_json::Value::Number(n) => n.as_f64().unwrap_or(0.0),
        serde_json::Value::String(s) => s.parse::<f64>().map_err(|_| {
            ToolInputError::new(format!("Parameter '{}' must be a valid number", key))
        })?,
        serde_json::Value::Bool(b) => if *b { 1.0 } else { 0.0 },
        _ => {
            return Err(ToolInputError::new(format!("Parameter '{}' must be a number", key)));
        }
    };

    if let Some(min) = options.min {
        if num_value < min {
            return Err(ToolInputError::new(format!(
                "Parameter '{}' must be at least {}", key, min
            )));
        }
    }

    if let Some(max) = options.max {
        if num_value > max {
            return Err(ToolInputError::new(format!(
                "Parameter '{}' must be at most {}", key, max
            )));
        }
    }

    Ok(num_value)
}

/// Read and validate a path parameter
pub fn read_path_param(
    params: &HashMap<String, serde_json::Value>,
    key: &str,
    options: PathParamOptions,
) -> Result<String, ToolInputError> {
    let string_value = read_string_param(
        params,
        key,
        StringParamOptions {
            required: options.required,
            trim: true,
            allow_empty: false,
            ..Default::default()
        },
    )?;

    if string_value.is_empty() {
        return Ok(string_value);
    }

    let path = Path::new(&string_value);

    if options.must_exist {
        if !path.exists() {
            return Err(ToolInputError::new(format!(
                "Path '{}' does not exist", string_value
            )));
        }

        if options.must_be_file && !path.is_file() {
            return Err(ToolInputError::new(format!(
                "Path '{}' must be a file", string_value
            )));
        }

        if options.must_be_dir && !path.is_dir() {
            return Err(ToolInputError::new(format!(
                "Path '{}' must be a directory", string_value
            )));
        }
    }

    if !options.allow_relative && path.is_relative() {
        return Err(ToolInputError::new(format!(
            "Path '{}' must be absolute", string_value
        )));
    }

    Ok(string_value)
}

/// Sanitize tool result images by removing or blurring sensitive content
pub async fn sanitize_tool_result_images(
    _result: &mut serde_json::Value,
    _limits: Option<&ImageSanitizationLimits>,
) -> Result<(), ToolInputError> {
    // TODO: Implement image sanitization
    // This would involve:
    // 1. Extracting image data from the result
    // 2. Applying sanitization (blur, redact, etc.)
    // 3. Updating the result with sanitized images
    Ok(())
}

/// Image sanitization limits
#[derive(Debug, Clone)]
pub struct ImageSanitizationLimits {
    pub max_width: Option<u32>,
    pub max_height: Option<u32>,
    pub quality: Option<u8>,
    pub format: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_read_string_param_required() {
        let mut params = HashMap::new();
        params.insert("name".to_string(), serde_json::Value::String("test".to_string()));

        let result = read_string_param(
            &params,
            "name",
            StringParamOptions {
                required: true,
                trim: true,
                ..Default::default()
            },
        );
        assert_eq!(result.unwrap(), "test");
    }

    #[test]
    fn test_read_string_param_missing_required() {
        let params = HashMap::new();

        let result = read_string_param(
            &params,
            "name",
            StringParamOptions {
                required: true,
                ..Default::default()
            },
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_read_boolean_param() {
        let mut params = HashMap::new();
        params.insert("enabled".to_string(), serde_json::Value::Bool(true));

        let result = read_boolean_param(
            &params,
            "enabled",
            BooleanParamOptions::default(),
        );
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_read_number_param() {
        let mut params = HashMap::new();
        params.insert("count".to_string(), serde_json::json!(42.5));

        let result = read_number_param(
            &params,
            "count",
            NumberParamOptions {
                min: Some(0.0),
                max: Some(100.0),
                ..Default::default()
            },
        );
        assert_eq!(result.unwrap(), 42.5);
    }

    #[test]
    fn test_create_action_gate() {
        let mut actions = HashMap::new();
        actions.insert("read".to_string(), true);
        actions.insert("write".to_string(), false);

        let gate = create_action_gate(Some(actions));

        assert_eq!(gate("read", None), true);
        assert_eq!(gate("write", None), false);
        assert_eq!(gate("unknown", Some(true)), true);
    }
}