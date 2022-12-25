use crate::types::{types::Result, Env};

pub struct GitCommands {
    pub env: Env,
}

pub struct GitLogOptions {
    pub max_count: Option<u32>,
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

    pub fn git_log(&self, branch_name: String, options: GitLogOptions) -> Result<String> {
        let format = "%H %T %P %an %ae %ad \"%ar\" %cn %ce %cd \"%cr\" [%s]";
        let mut binding = std::process::Command::new("git");
        let output = binding
            .arg("log")
            .arg("--date=iso-local")
            .arg(format!("--pretty=format:\"{}\"", format))
            .arg("--date=format:%Y-%m-%d_%H:%M:%S")
            .arg(branch_name);

        // 取得するログ数の制限
        if options.max_count != None {
            output.arg(format!(
                "--max-count={}",
                options.max_count.map_or(10, |n| n)
            ));
        }

        let output = output.output()?;
        let stdout = std::str::from_utf8(&output.stdout)?;
        Ok(stdout.to_string())
    }
}

/// 動作確認するようで、テストケースは未実装
#[cfg(test)]
mod GitCommandsTest {
    use crate::{
        core::git_commands::GitLogOptions,
        types::{types::Result, Env},
    };

    use super::GitCommands;

    #[test]
    pub fn git_log_test() -> Result<()> {
        let command = GitCommands::new(Env { dir: None });
        let options = GitLogOptions { max_count: Some(2) };
        let result = command.git_log("main".to_string(), options)?;
        println!("{}", result);
        Ok(())
    }
}
