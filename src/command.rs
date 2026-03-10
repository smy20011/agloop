use std::path::{Path, PathBuf};

use chrono::Utc;
use duct::{Expression, cmd};

use crate::config::CommandConfig;

pub trait Command {
    fn cmd(&self) -> Expression;
    fn cmd_log(&self, dir: &Path) -> (Expression, PathBuf);
}

impl Command for CommandConfig {
    fn cmd(&self) -> Expression {
        cmd(&self.bin, &self.args)
    }

    fn cmd_log(&self, dir: &Path) -> (Expression, PathBuf) {
        let filename = format!("{}_{}.log", self.bin, Utc::now().timestamp());
        let full_path = dir.join(filename);
        (
            self.cmd().stderr_to_stdout().pipe(cmd!("tee", &full_path)),
            full_path,
        )
    }
}
