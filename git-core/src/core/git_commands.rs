use crate::types::{
    errors::{Error, ErrorDetail},
    types::Result,
    Env,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CommitHash {
    pub hash: String,
}

impl CommitHash {
    pub fn new(hash: String) -> Result<Self> {
        if hash.len() != 40 {
            return Err(Box::new(Error::InvalidValueError(ErrorDetail::new(
                "ハッシュ値が不正です",
                "ハッシュ値の値が40文字ではありません",
            ))));
        }
        Ok(Self { hash })
    }
}

#[derive(Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    pub head_hash: CommitHash,
}

impl Branch {
    pub fn new(name: String, head_hash: String) -> Result<Self> {
        let head_hash = CommitHash::new(head_hash)?;
        Ok(Self { name, head_hash })
    }
}

/// とりあえず最初はこれでやってみる
#[derive(Serialize, Deserialize)]
pub struct CommitInfo {
    pub commit_hash: CommitHash,
    pub parent_hash: CommitHash,
    pub author: String,
    pub committer: String,
    pub author_email: String,
    pub committer_email: String,
}

pub struct GitCommands {
    pub env: Env,
}

impl GitCommands {
    pub fn new(env: Env) -> Self {
        Self { env }
    }

    pub fn git_branch(&self) -> Result<String> {
        let output = std::process::Command::new("git").arg("branch").output()?;
        let stdout = std::str::from_utf8(&output.stdout)?;
        Ok(stdout.to_string())
    }

    pub fn git_log(&self, branch_name: String) -> Result<String> {
        let format = "%H %T %P %an %ae %ad \"%ar\" %cn %ce %cd \"%cr\" [%s]";
        let output = std::process::Command::new("git")
            .arg("log")
            .arg("--date=iso-local")
            .arg(format!("--pretty=format:\"{}\"", format))
            .arg("--date=format:%Y-%m-%d_%H:%M:%S")
            .arg(branch_name)
            .output()?;
        let stdout = std::str::from_utf8(&output.stdout)?;
        Ok(stdout.to_string())
    }
}

/// 動作確認するようで、テストケースは未実装
#[cfg(test)]
mod GitCommandsTest {
    use crate::types::{types::Result, Env};

    use super::GitCommands;

    #[test]
    pub fn git_log_test() -> Result<()> {
        let command = GitCommands::new(Env { dir: None });
        let result = command.git_log("main".to_string())?;
        println!("{}", result);
        Ok(())
    }
}
