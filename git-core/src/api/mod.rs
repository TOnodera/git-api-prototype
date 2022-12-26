use crate::{
    core::{
        converters::Converters,
        git_commands::{GitCommands, GitLogOptions},
    },
    types::{
        errors::{Error, ErrorDetail},
        types::Result,
        Env,
    },
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
pub struct GitApi {
    commands: GitCommands,
    converter: Converters,
}

impl GitApi {
    pub fn new(env: Env) -> Self {
        Self {
            commands: GitCommands::new(env),
            converter: Converters::new(),
        }
    }
}

trait GitApiInterface {
    // ブランチ情報を取得する
    fn get_branches(&self) -> Result<Vec<Branch>>;
    // 引数に指定されたハッシュからたどれるすべてのコミットを返す
    fn get_logs(&self, hash: CommitHash, options: GitLogOptions) -> Result<Vec<CommitHash>>;
    // コミット情報を取得する
    fn get_commit_info(&self, hash: CommitHash) -> Result<CommitInfo>;
}

impl GitApiInterface for GitApi {
    // ブランチの一覧と先頭コミットを取得する
    fn get_branches(&self) -> Result<Vec<Branch>> {
        // ブランチ名を取得する
        let output = self.commands.git_branch()?;
        let branch_names = self.converter.git_branch(output)?;

        let mut branches = Vec::<Branch>::new();
        for branch_name in branch_names {
            let options = GitLogOptions { max_count: Some(1) };
            let logs = self
                .converter
                .git_log(self.commands.git_log(branch_name.clone(), options)?)?;

            for log in logs {
                let head_hash = log[0].to_string();
                let branch = Branch::new(branch_name.clone(), head_hash)?;
                branches.push(branch);
            }
        }

        Ok(branches)
    }

    // 指定されたコミットからたどれるログを取得する
    fn get_logs(&self, hash: CommitHash, options: GitLogOptions) -> Result<Vec<CommitHash>> {
        todo!()
    }

    // 特定のコミットのコミット情報を取得する
    fn get_commit_info(&self, hash: CommitHash) -> Result<CommitInfo> {
        todo!()
    }
}

#[cfg(test)]
mod GitApiTest {
    use crate::types::Env;

    use super::{GitApi, GitApiInterface};

    #[test]
    fn get_branches_test() {
        let api = GitApi::new(Env::new(None));
        let value = api.get_branches();
    }
}
