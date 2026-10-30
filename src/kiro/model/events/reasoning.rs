//! 推理内容事件
//!
//! 处理 reasoningContentEvent 类型的事件（模型的思考过程）。
//! 目前只做统计，不转发给客户端。

use serde::Deserialize;

use crate::kiro::parser::error::ParseResult;
use crate::kiro::parser::frame::Frame;

use super::base::EventPayload;

/// 推理内容事件
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ReasoningContentEvent {
    /// 思考内容片段
    #[serde(default)]
    pub text: String,
}

impl EventPayload for ReasoningContentEvent {
    fn from_frame(frame: &Frame) -> ParseResult<Self> {
        // 只用于统计：格式不符时按空值处理，不影响事件流
        Ok(frame.payload_as_json().unwrap_or_default())
    }
}
