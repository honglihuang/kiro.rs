//! 元数据事件
//!
//! 处理 metadataEvent 类型的事件。上游只对较新的客户端版本发送，
//! 内容是停止原因（END_TURN / TOOL_USE / MAX_TOKENS / CONTENT_FILTERED 等），偶尔带 tokenUsage。

use serde::Deserialize;

use crate::kiro::parser::error::ParseResult;
use crate::kiro::parser::frame::Frame;

use super::base::EventPayload;

/// 元数据事件
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataEvent {
    /// 停止原因，例如 END_TURN / TOOL_USE / MAX_TOKENS / CONTENT_FILTERED
    #[serde(default)]
    pub stop_reason: Option<String>,
    /// 停止详情；内容过滤时包含 refusal.category / refusal.explanation
    #[serde(default)]
    pub stop_details: Option<serde_json::Value>,
    /// 上游给出的 token 用量（大多数响应没有）
    #[serde(default)]
    pub token_usage: Option<serde_json::Value>,
}

impl EventPayload for MetadataEvent {
    fn from_frame(frame: &Frame) -> ParseResult<Self> {
        // 目前只用于日志：格式不符时按空值处理，不影响事件流
        Ok(frame.payload_as_json().unwrap_or_default())
    }
}

impl MetadataEvent {
    /// 内容过滤拒答的类别，例如 CYBER
    pub fn refusal_category(&self) -> Option<&str> {
        self.stop_details
            .as_ref()?
            .get("refusal")?
            .get("category")?
            .as_str()
    }

    /// 内容过滤拒答的说明文字（上游给的固定提示，不含对话内容）
    pub fn refusal_explanation(&self) -> Option<&str> {
        self.stop_details
            .as_ref()?
            .get("refusal")?
            .get("explanation")?
            .as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_content_filtered_refusal() {
        let event: MetadataEvent = serde_json::from_str(
            r#"{"stopDetails":{"refusal":{"category":"CYBER","explanation":"cannot continue"}},"stopReason":"CONTENT_FILTERED"}"#,
        )
        .unwrap();
        assert_eq!(event.stop_reason.as_deref(), Some("CONTENT_FILTERED"));
        assert_eq!(event.refusal_category(), Some("CYBER"));
        assert_eq!(event.refusal_explanation(), Some("cannot continue"));
    }

    #[test]
    fn plain_stop_reason_has_no_refusal() {
        let event: MetadataEvent = serde_json::from_str(r#"{"stopReason":"END_TURN"}"#).unwrap();
        assert_eq!(event.stop_reason.as_deref(), Some("END_TURN"));
        assert_eq!(event.refusal_category(), None);
        assert!(event.token_usage.is_none());
    }
}
