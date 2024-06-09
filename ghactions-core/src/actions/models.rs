//! # Models

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

use crate::ActionsError;

const GHACTIONS_ROOT: &str = env!("CARGO_MANIFEST_DIR");

/// Action YAML file structure
///
/// https://docs.github.com/en/actions/creating-actions/metadata-syntax-for-github-actions
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionYML {
    /// Action Path
    #[serde(skip)]
    pub path: Option<PathBuf>,

    /// Action Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Action Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Action Author
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    /// Action Branding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branding: Option<ActionBranding>,

    /// Action Inputs
    pub inputs: HashMap<String, ActionInput>,
    /// Action Outputs
    pub outputs: HashMap<String, ActionOutput>,

    /// Action Runs
    pub runs: ActionRuns,
}

impl Default for ActionYML {
    fn default() -> Self {
        ActionYML {
            path: None,
            name: Some(env!("CARGO_PKG_NAME").to_string()),
            description: None,
            author: None,
            branding: None,
            inputs: HashMap::new(),
            outputs: HashMap::new(),
            runs: ActionRuns::default(),
        }
    }
}

impl ActionYML {
    /// Load the Action YAML file
    pub fn load_action(path: String) -> Result<ActionYML, Box<dyn std::error::Error>> {
        let fhandle = std::fs::File::open(&path)?;
        let mut action_yml: ActionYML = serde_yaml::from_reader(fhandle)?;
        action_yml.path = Some(PathBuf::from(path.clone()));
        Ok(action_yml)
    }

    /// Write the Action YAML file
    pub fn write(&self) -> Result<PathBuf, ActionsError> {
        if let Some(ref path) = self.path {
            if !path.exists() {
                let parent = path.parent().unwrap();
                std::fs::create_dir_all(parent)
                    .map_err(|err| ActionsError::IOError(err.to_string()))?;
            }

            // Create or Open the file
            let fhandle = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(path)
                .map_err(|err| ActionsError::IOError(err.to_string()))?;

            serde_yaml::to_writer(fhandle, self)
                .map_err(|err| ActionsError::IOError(err.to_string()))?;

            Ok(path.clone())
        } else {
            Err(ActionsError::NotImplemented)
        }
    }
}

/// Action Input structure
#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ActionInput {
    /// Input Type
    #[serde(skip)]
    pub r#type: String,

    /// Input Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Input Required or not
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// Input Default value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    /// Deprecation Message
    #[serde(rename = "deprecationMessage", skip_serializing_if = "Option::is_none")]
    pub deprecation_message: Option<String>,
}

/// Action Output structure
#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ActionOutput {
    /// Output Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Action Branding
///
/// https://docs.github.com/en/actions/creating-actions/metadata-syntax-for-github-actions#branding
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionBranding {
    /// Action Color
    pub color: String,
    /// Action Icon
    pub icon: String,
}

/// Action Runs structure
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionRuns {
    /// Action Name
    pub using: String,
    /// Docker Image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<PathBuf>,
    /// Docker Arguments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
}

impl Default for ActionRuns {
    fn default() -> Self {
        Self {
            using: String::from("docker"),
            image: Some(PathBuf::from("./Dockerfile")),
            args: None,
        }
    }
}
