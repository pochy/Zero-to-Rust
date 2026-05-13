use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Operation {
    Set,
    Get,
    Delete,
    Exists,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CommandRequest {
    pub op: Operation,
    pub key: String,
    #[serde(default)]
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ResponseBody {
    Ok,
    Value { value: String },
    NotFound,
    Exists { exists: bool },
    Error { message: String },
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum AppError {
    #[error("invalid json: {0}")]
    InvalidJson(String),
    #[error("missing value for set")]
    MissingValue,
    #[error("failed to serialize response: {0}")]
    Serialize(String),
}

#[derive(Debug, Default)]
pub struct Store {
    data: HashMap<String, String>,
}

impl Store {
    pub fn handle(&mut self, request: CommandRequest) -> Result<ResponseBody, AppError> {
        match request.op {
            Operation::Set => {
                let value = request.value.ok_or(AppError::MissingValue)?;
                self.data.insert(request.key, value);
                Ok(ResponseBody::Ok)
            }
            Operation::Get => match self.data.get(&request.key) {
                Some(value) => Ok(ResponseBody::Value {
                    value: value.clone(),
                }),
                None => Ok(ResponseBody::NotFound),
            },
            Operation::Delete => {
                if self.data.remove(&request.key).is_some() {
                    Ok(ResponseBody::Ok)
                } else {
                    Ok(ResponseBody::NotFound)
                }
            }
            Operation::Exists => Ok(ResponseBody::Exists {
                exists: self.data.contains_key(&request.key),
            }),
        }
    }
}

pub fn handle_json(store: &mut Store, input: &str) -> Result<String, AppError> {
    let request: CommandRequest =
        serde_json::from_str(input).map_err(|error| AppError::InvalidJson(error.to_string()))?;
    let response = store.handle(request)?;
    serde_json::to_string(&response).map_err(|error| AppError::Serialize(error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_set_and_get_json() {
        let mut store = Store::default();

        assert_eq!(
            handle_json(&mut store, r#"{"op":"set","key":"name","value":"Rust"}"#).unwrap(),
            r#"{"status":"ok"}"#
        );
        assert_eq!(
            handle_json(&mut store, r#"{"op":"get","key":"name"}"#).unwrap(),
            r#"{"status":"value","value":"Rust"}"#
        );
    }

    #[test]
    fn classifies_missing_value() {
        let mut store = Store::default();

        assert_eq!(
            handle_json(&mut store, r#"{"op":"set","key":"name"}"#).unwrap_err(),
            AppError::MissingValue
        );
    }

    #[test]
    fn classifies_invalid_json() {
        let mut store = Store::default();

        assert!(matches!(
            handle_json(&mut store, "not json").unwrap_err(),
            AppError::InvalidJson(_)
        ));
    }
}
