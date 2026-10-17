//! 带会话身份的 Linux 事件封套。
use super::LinuxEvent;
use serde::{Deserialize, Serialize};
use yiyan_platform::protocol::SessionId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinuxRequest {
    pub session: SessionId,

    pub event: LinuxEvent,
}
