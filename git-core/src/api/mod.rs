use crate::{
    core::git_commands::{Branch, CommitHash, CommitInfo},
    types::types::Result,
};
pub struct GitApi;

trait GitApiInterface {
    // ブランチ情報を取得する
    fn get_branches() -> Result<Vec<Branch>>;
    // 引数に指定されたハッシュからたどれるすべてのコミットを返す
    fn get_commits(hash: CommitHash) -> Result<Vec<CommitHash>>;
    // コミット情報を取得する
    fn get_commit_info(hash: CommitHash) -> Result<CommitInfo>;
}

impl GitApiInterface for GitApi {
    fn get_branches() -> Result<Vec<Branch>> {
        todo!()
    }

    fn get_commits(hash: CommitHash) -> Result<Vec<CommitHash>> {
        todo!()
    }

    fn get_commit_info(hash: CommitHash) -> Result<CommitInfo> {
        todo!()
    }
}
