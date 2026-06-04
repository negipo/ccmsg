use serde::Serialize;

/// messages テーブルの 1 行（未読として claim された結果含む）
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Message {
    pub id: i64,
    pub from_agent: String,
    pub to_agent: String,
    pub body: String,
    pub created_at: String,
    pub read_at: Option<String>,
}

/// 自己登録（agents 簿への自分の登録）の結果
#[derive(Debug, Clone, PartialEq)]
pub enum RegisterOutcome {
    /// 新規に insert した
    Registered,
    /// 既に同じ name / project_path で登録済み（書き込みなし）
    Unchanged,
    /// 同じ name が別の project_path で登録済み（衝突）
    Collision { existing_path: String },
}

/// inbox / wait 一回分の受信結果
#[derive(Debug)]
pub enum ReceiveOutcome {
    /// 正常受信（claim 済み＝既読化済み）
    Claimed(Vec<Message>),
    /// 同名 identity 衝突。未読は表示するが既読化しない
    Collision {
        existing_path: String,
        unread: Vec<Message>,
    },
}
