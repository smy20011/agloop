use std::{collections::HashSet, path::PathBuf};

use anyhow::{Result, anyhow};
use regex::RegexBuilder;
use serde::{Deserialize, Serialize};

const OPEN_TAG: &str = "open";
const DONE_TAG: &str = "done";

pub struct TaskManager {
    task_root: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Default)]
pub struct Task {
    pub name: String,
    #[serde(default)]
    pub deps: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default = "default_priority")]
    pub priority: u32,
}

fn default_priority() -> u32 {
    return 3;
}

impl TaskManager {
    pub fn get_tasks() -> Vec<Task> {
        unimplemented!()
    }

    /// Returns a list of unimplemented tasks, ordered by priority.
    pub fn get_open_tasks() -> Vec<Task> {
        unimplemented!()
    }

    fn parse_task(content: &str) -> Result<Task> {
        let re = RegexBuilder::new(r"^---(.*)---$")
            .multi_line(true)
            .dot_matches_new_line(true)
            .build()?;
        if let Some(m) = re.captures(content) {
            let yml = m.get(1).unwrap().as_str();
            Ok(serde_yaml::from_str(yml)?)
        } else {
            Err(anyhow!("Cannot find frontmatter"))
        }
    }

    fn get_open_task_internal(tasks: &Vec<Task>) -> Vec<Task> {
        let done_task: HashSet<&str> = tasks
            .iter()
            .filter(|t| t.tags.contains(&DONE_TAG.to_string()))
            .map(|t| t.name.as_str())
            .collect();

        let mut tasks: Vec<Task> = tasks
            .iter()
            .filter(|t| t.tags.contains(&OPEN_TAG.to_string()))
            .filter(|t| t.deps.iter().all(|d| done_task.contains(d.as_str())))
            .cloned()
            .collect();
        tasks.sort_by_key(|t| t.priority);
        tasks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_task() {
        let content = "---
name: test
tags: [task, open]
priority: 0
---";
        assert_eq!(
            TaskManager::parse_task(content).unwrap(),
            Task {
                name: "test".into(),
                tags: vec!["task".into(), "open".into()],
                deps: vec![],
                priority: 0,
            }
        );
    }

    #[test]
    fn reject_task_without_name() {
        let content = "---
tags: [task, open]
priority: 0
---";
        assert!(TaskManager::parse_task(content).is_err());
    }

    #[test]
    fn test_get_done_task() {
        let tasks = vec![
            Task {
                name: "test1".into(),
                tags: vec!["task".into(), "done".into()],
                ..Default::default()
            },
            Task {
                name: "test_low_pri".into(),
                tags: vec!["task".into(), "open".into()],
                deps: vec!["test1".into()],
                priority: 3,
                ..Default::default()
            },
            Task {
                name: "test2".into(),
                tags: vec!["task".into(), "open".into()],
                deps: vec!["test1".into()],
                ..Default::default()
            },
            Task {
                name: "test3".into(),
                tags: vec!["task".into(), "open".into()],
                deps: vec!["test2".into()],
                ..Default::default()
            },
        ];

        let task_names: Vec<_> = TaskManager::get_open_task_internal(&tasks)
            .iter()
            .map(|t| t.name.clone())
            .collect();

        assert_eq!(task_names, vec!["test2", "test_low_pri"]);
    }
}
