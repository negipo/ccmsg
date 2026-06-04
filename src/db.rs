use anyhow::{bail, Context, Result};
use rusqlite::{params, Connection};

use crate::models::{Message, ReceiveOutcome, RegisterOutcome};

/// プロジェクトルートのパスから識別子（最終パス要素）を得る
pub fn basename(project_path: &str) -> String {
    project_path
        .trim_end_matches('/')
        .rsplit('/')
        .next()
        .unwrap_or(project_path)
        .to_string()
}

pub struct Database {
    conn: Connection,
}

impl Database {
    /// 環境変数 CCMSG_DB_PATH があればそこを、なければ data_dir/ccmsg/messages.db を開く
    pub fn open() -> Result<Self> {
        let db_path = match std::env::var("CCMSG_DB_PATH") {
            Ok(p) => std::path::PathBuf::from(p),
            Err(_) => {
                let data_dir = dirs::data_dir()
                    .context("データディレクトリが見つかりません")?
                    .join("ccmsg");
                std::fs::create_dir_all(&data_dir)?;
                data_dir.join("messages.db")
            }
        };
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(db_path)?;
        Self::init(conn)
    }

    pub fn in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        Self::init(conn)
    }

    fn init(conn: Connection) -> Result<Self> {
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.busy_timeout(std::time::Duration::from_secs(5))?;
        let db = Self { conn };
        db.create_tables()?;
        Ok(db)
    }

    fn create_tables(&self) -> Result<()> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS messages (
                id         INTEGER PRIMARY KEY AUTOINCREMENT,
                from_agent TEXT NOT NULL,
                to_agent   TEXT NOT NULL,
                body       TEXT NOT NULL,
                created_at TEXT NOT NULL,
                read_at    TEXT
            );
            CREATE INDEX IF NOT EXISTS idx_unread ON messages(to_agent, read_at);
            CREATE TABLE IF NOT EXISTS agents (
                name         TEXT PRIMARY KEY,
                project_path TEXT NOT NULL
            );",
        )?;
        Ok(())
    }

    /// 既知の宛先名一覧（昇順）
    pub fn list_agents(&self) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT name FROM agents ORDER BY name")?;
        let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
        let mut names = Vec::new();
        for r in rows {
            names.push(r?);
        }
        Ok(names)
    }

    /// 自分（name / project_path）を agents 簿へ登録する。
    /// - 未登録なら insert（Registered）
    /// - 同 name 同 project_path なら何もしない（Unchanged）
    /// - 同 name 別 project_path なら衝突（Collision、書き込みなし）
    pub fn register_self(&self, name: &str, project_path: &str) -> Result<RegisterOutcome> {
        let existing: Option<String> = {
            let mut stmt = self
                .conn
                .prepare("SELECT project_path FROM agents WHERE name = ?1")?;
            let mut rows = stmt.query(params![name])?;
            match rows.next()? {
                Some(row) => Some(row.get(0)?),
                None => None,
            }
        };

        match existing {
            None => {
                self.conn.execute(
                    "INSERT INTO agents (name, project_path) VALUES (?1, ?2)",
                    params![name, project_path],
                )?;
                Ok(RegisterOutcome::Registered)
            }
            Some(path) if path == project_path => Ok(RegisterOutcome::Unchanged),
            Some(path) => Ok(RegisterOutcome::Collision {
                existing_path: path,
            }),
        }
    }

    /// 宛先 to へ from から body を投函する。
    /// 宛先未知・自分宛・空本文はエラーで停止する。
    pub fn send_message(&self, from: &str, to: &str, body: &str) -> Result<()> {
        if body.trim().is_empty() {
            bail!("cannot send an empty message");
        }
        if from == to {
            bail!("cannot send a message to yourself");
        }
        let known: bool = {
            let mut stmt = self.conn.prepare("SELECT 1 FROM agents WHERE name = ?1")?;
            stmt.exists(params![to])?
        };
        if !known {
            bail!(
                "destination '{}' is unknown. Once they run /ccmsg to join, you can send to them",
                to
            );
        }
        let now = chrono::Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO messages (from_agent, to_agent, body, created_at, read_at)
             VALUES (?1, ?2, ?3, ?4, NULL)",
            params![from, to, body, now],
        )?;
        Ok(())
    }

    /// 自分宛の未読を read_at セットしつつ取り出す（アトミックな claim）
    pub fn claim_unread(&self, name: &str) -> Result<Vec<Message>> {
        let now = chrono::Utc::now().to_rfc3339();
        let mut stmt = self.conn.prepare(
            "UPDATE messages SET read_at = ?1
             WHERE to_agent = ?2 AND read_at IS NULL
             RETURNING id, from_agent, to_agent, body, created_at, read_at",
        )?;
        let rows = stmt.query_map(params![now, name], Self::row_to_message)?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r?);
        }
        Ok(out)
    }

    /// 自分宛の未読を既読化せずに覗く（衝突時の表示用）
    pub fn peek_unread(&self, name: &str) -> Result<Vec<Message>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, from_agent, to_agent, body, created_at, read_at
             FROM messages
             WHERE to_agent = ?1 AND read_at IS NULL
             ORDER BY id",
        )?;
        let rows = stmt.query_map(params![name], Self::row_to_message)?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r?);
        }
        Ok(out)
    }

    /// inbox / wait 一回分の受信。自己登録の結果で claim か peek を分岐する。
    pub fn receive_once(&self, name: &str, project_path: &str) -> Result<ReceiveOutcome> {
        match self.register_self(name, project_path)? {
            RegisterOutcome::Collision { existing_path } => Ok(ReceiveOutcome::Collision {
                existing_path,
                unread: self.peek_unread(name)?,
            }),
            RegisterOutcome::Registered | RegisterOutcome::Unchanged => {
                Ok(ReceiveOutcome::Claimed(self.claim_unread(name)?))
            }
        }
    }

    /// messages / agents を全消去して DB を初期化する
    pub fn reset(&self) -> Result<()> {
        self.conn
            .execute_batch("BEGIN; DELETE FROM messages; DELETE FROM agents; COMMIT;")?;
        Ok(())
    }

    fn row_to_message(row: &rusqlite::Row) -> rusqlite::Result<Message> {
        Ok(Message {
            id: row.get(0)?,
            from_agent: row.get(1)?,
            to_agent: row.get(2)?,
            body: row.get(3)?,
            created_at: row.get(4)?,
            read_at: row.get(5)?,
        })
    }
}
