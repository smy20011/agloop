use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CommandConfig {
    /// Path to the agent binary.
    pub bin: String,
    /// Additional commands that passed to the agent.
    #[serde(default)]
    pub args: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VcsConfig {
    pub commit: CommandConfig,
    pub abandon: CommandConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    /// Directory that holds all the (notes, bugs, tasks, project description)
    pub note: String,
    /// Name of agent to use
    pub agent: String,
    /// Agent configs
    pub agents: HashMap<String, CommandConfig>,
    /// VCS configs
    pub vcs: VcsConfig,
}

#[cfg(test)]
mod test {
    use super::*;

    const TOML_EXAMPLE: &'static str = r#"
note = "./notes"
agent = "codex"

[agents.codex]
bin = "codex"

[vcs.commit]
bin = "jj"
args = ["commit", "-m"]

[vcs.abandon]
bin = "jj"
args = ["abandon"]
"#;

    #[test]
    fn test_parse_toml() {
        let config: Config = toml::from_str(TOML_EXAMPLE).unwrap();
        assert_eq!(config.note, "./notes");
    }
}
