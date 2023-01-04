use regex::Regex;

use crate::types::types::Result;

use super::git_commands::GitLogOptions;

pub struct Converters;
/// git_commands.rsで取得できるコマンド出力の文字列をもう一段上の型に変換していく
impl Converters {
    pub fn new() -> Self {
        Self
    }
    pub fn git_branch(&self, input: String) -> Result<Vec<String>> {
        let mut branches = Vec::<String>::new();
        for branch in input.split("\n").into_iter() {
            if branch.is_empty() {
                continue;
            }
            let re = Regex::new("^* (.+)$")?;
            let cap = re.captures(branch);
            match cap {
                Some(name) => {
                    let name = name.get(1).map(|m| m.as_str());
                    match name {
                        Some(name) => branches.push(name.trim().to_string()),
                        None => todo!(),
                    }
                }
                None => {
                    branches.push(branch.trim().to_string());
                }
            }
        }

        Ok(branches)
    }

    pub fn git_log(&self, input: String) -> Result<Vec<Vec<String>>> {
        let mut git_logs = Vec::<Vec<String>>::new();
        let re = Regex::new(
            r#"(.+?) (.+?) (.*) (.+) (.+) (.+) "(.+?)" (.+?) (.+?) (.+?) "(.+?)" \[(.+)\]"#,
        )?;
        for line in input.split("\n") {
            let mut git_log = Vec::<String>::new();
            let line = Regex::new("^ *\"(.+)\" *$")?.replace(&line, "$1");
            if let Some(value) = re.captures(&line) {
                for i in 1..=value.len() {
                    value.get(i).map(|m| git_log.push(m.as_str().to_string()));
                }
                git_logs.push(git_log);
            }
        }
        Ok(git_logs)
    }
}

#[cfg(test)]
mod ConvertersTest {
    use crate::types::types::Result;

    use super::Converters;

    #[test]
    fn git_branch_test() -> Result<()> {
        let command_output = " feature/test1\n feature/test2\n develop\n* main\n";
        let results = Converters::new().git_branch(command_output.to_string())?;
        assert_eq!(results.len(), 4);
        assert_eq!(results[0].to_string(), "feature/test1");
        assert_eq!(results[1].to_string(), "feature/test2");
        assert_eq!(results[2].to_string(), "develop");
        assert_eq!(results[3].to_string(), "main");
        Ok(())
    }

    #[test]
    fn git_log_test() -> Result<()> {
        // コメントに開業を入れたテストケースも必要
        let command_output = r#"
        "2fba33d2a1f23eee4a5e2b855704b24a2cc6742c a07ab05e0d596de6615072385ba27d8e7544d398 ecd55fdd0397a8fc82ecfa14c8edba274e8425d4 example example@example.com 2022-08-30_12:12:08 "4 months ago" example example@example.com 2022-08-30_12:12:08 "4 months ago" [anyhowいれた]"
        "ecd55fdd0397a8fc82ecfa14c8edba274e8425d4 69c73cf39fcfcbc578d4445ec83a3e2fe21d9d91 ea5705354c4963bdef986908df68f7970bc9855d example example@example.com 2022-08-30_12:01:43 "4 months ago" example example@example.com 2022-08-30_12:01:43 "4 months ago" [thiserror試してみ多度hoge]"
        "ea5705354c4963bdef986908df68f7970bc9855d 2f04d08dd0f399e5367a44861b09e4b049c8fa2f 3d23d550a08b746d9f72225668e9bd0e5cea63c1 example example@example.com 2022-08-29_13:24:22 "4 months ago" example example@example.com 2022-08-29_13:24:22 "4 months ago" [systemstat info]"
        "3d23d550a08b746d9f72225668e9bd0e5cea63c1 8e7b069239d82efa567fc15b7e7282a8c9cc0528 9e58a45ecbbd9fe7b5a667be2d684b9f090fb078 example example@example.com 2022-08-24_10:04:01 "4 months ago" example example@example.com 2022-08-24_10:04:01 "4 months ago" [ここ]"
        "9e58a45ecbbd9fe7b5a667be2d684b9f090fb078 3c27d005440da719ec349b0c9cd9b8964da2df08 8be8845e185e822a3a63b1408afdbc67ca8d5658 example example@example.com 2022-08-17_12:05:06 "4 months ago" example example@example.com 2022-08-17_12:05:06 "4 months ago" [dynで動的ディスパッチ]"
        "8be8845e185e822a3a63b1408afdbc67ca8d5658 c12d611b33c68a547f82321b639137eea3c1cc0b 8f24e95417cd6c7a61fe95083c016fcce8fc8cdd TOnodera 46294684+TOnodera@users.noreply.github.com 2022-08-05_18:34:54 "5 months ago" GitHub noreply@github.com 2022-08-05_18:34:54 "5 months ago" [md修正]"
        "8f24e95417cd6c7a61fe95083c016fcce8fc8cdd 6051399b9edd81d151a08b83600595c59defc270 d4a9d029a5ea861a9989807c04f0f9097e5ec6a9 example example@example.com 2022-08-05_09:32:53 "5 months ago" example example@example.com 2022-08-05_09:32:53 "5 months ago" [README.dm追加]"
        "d4a9d029a5ea861a9989807c04f0f9097e5ec6a9 edf8dc7247cd6c72de826a9b109a073366329ce0  example example@example.com 2022-08-05_09:32:08 "5 months ago" example example@example.com 2022-08-05_09:32:08 "5 months ago" [最初のコミット]"
        "#;
        let results = Converters::new().git_log(command_output.to_string())?;

        // とりあえず最初と最後
        let first_commit = &results[7];
        assert_eq!(first_commit[0], "d4a9d029a5ea861a9989807c04f0f9097e5ec6a9");
        assert_eq!(first_commit[1], "edf8dc7247cd6c72de826a9b109a073366329ce0");
        assert_eq!(first_commit[2], "");
        assert_eq!(first_commit[3], "example");
        assert_eq!(first_commit[4], "example@example.com");
        assert_eq!(first_commit[5], "2022-08-05_09:32:08");
        assert_eq!(first_commit[6], "5 months ago");
        assert_eq!(first_commit[7], "example");
        assert_eq!(first_commit[8], "example@example.com");
        assert_eq!(first_commit[9], "2022-08-05_09:32:08");
        assert_eq!(first_commit[10], "5 months ago");
        assert_eq!(first_commit[11], "最初のコミット");

        let last_commit = &results[0];
        assert_eq!(last_commit[0], "2fba33d2a1f23eee4a5e2b855704b24a2cc6742c");
        assert_eq!(last_commit[1], "a07ab05e0d596de6615072385ba27d8e7544d398");
        assert_eq!(last_commit[2], "ecd55fdd0397a8fc82ecfa14c8edba274e8425d4");
        assert_eq!(last_commit[3], "example");
        assert_eq!(last_commit[4], "example@example.com");
        assert_eq!(last_commit[5], "2022-08-30_12:12:08");
        assert_eq!(last_commit[6], "4 months ago");
        assert_eq!(last_commit[7], "example");
        assert_eq!(last_commit[8], "example@example.com");
        assert_eq!(last_commit[9], "2022-08-30_12:12:08");
        assert_eq!(last_commit[10], "4 months ago");
        assert_eq!(last_commit[11], "anyhowいれた");
        Ok(())
    }
}
