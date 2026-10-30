//! 计费事件
//!
//! 处理 meteringEvent 类型的事件

use serde::Deserialize;

use crate::kiro::parser::error::ParseResult;
use crate::kiro::parser::frame::Frame;

use super::base::EventPayload;

/// 计费事件：本次上游响应消耗的额度
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeteringEvent {
    /// 消耗量（单位为 credit）
    #[serde(default)]
    pub usage: Option<f64>,
}

impl EventPayload for MeteringEvent {
    fn from_frame(frame: &Frame) -> ParseResult<Self> {
        // 只用于统计：格式不符时按空值处理，不影响事件流
        Ok(frame.payload_as_json().unwrap_or_default())
    }
}
