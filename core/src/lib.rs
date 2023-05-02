pub mod apk;

use serde::{Deserialize, Serialize};

// fn install_node(apk: &ApkPackageListLayer) {
//     apk.add("nodejs");
//     apk.add("npm");
// }

// fn install_python3(apk: &ApkPackageListLayer) {
//     apk.add("python3");
// }

// fn install_rustup(apk: &ApkPackageListLayer, version: &str) -> String {
//     let cache_key = format!("rust-{}", version);
//     apk.add("curl");
//     step(
//         "install_rustup",
//         [
//             "curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain $version",
//             r#"source "$HOME/.cargo/env"#,
//         ],
//         [&cache_key],
//     );

//     cache_key
// }

pub fn step<Command: AsRef<str>, Dep: Into<Dependency>>(
    title: impl AsRef<str>,
    commands: impl IntoIterator<Item = Command>,
    dependencies: impl IntoIterator<Item = Dep>,
) -> Step {
    Step::new(title, commands, dependencies)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Step {
    pub hash: String,
    pub title: String,
    pub commands: Vec<String>,
    pub dependencies: Vec<Dependency>,
}
#[derive(Serialize)]
pub struct StepWithoutHash {
    title: String,
    commands: Vec<String>,
    dependencies: Vec<Dependency>,
}

impl Step {
    fn new<Command: AsRef<str>, Dep: Into<Dependency>>(
        title: impl AsRef<str>,
        commands: impl IntoIterator<Item = Command>,
        dependencies: impl IntoIterator<Item = Dep>,
    ) -> Self {
        let step_without_hash = StepWithoutHash {
            title: title.as_ref().to_string(),
            commands: commands
                .into_iter()
                .map(|c| c.as_ref().to_string())
                .collect(),
            dependencies: dependencies.into_iter().map(|d| d.into()).collect(),
        };

        let hash = sha256::digest(serde_yaml::to_string(&step_without_hash).unwrap());

        let step = Self {
            hash,
            title: step_without_hash.title,
            commands: step_without_hash.commands,
            dependencies: step_without_hash.dependencies,
        };

        serde_yaml::to_writer(std::io::stdout(), &vec![&step]).unwrap();

        step
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Dependency {
    String(String),
    Step { hash: String },
}

impl From<String> for Dependency {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<&String> for Dependency {
    fn from(s: &String) -> Self {
        Self::String(s.to_string())
    }
}

impl From<&str> for Dependency {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

impl From<Step> for Dependency {
    fn from(s: Step) -> Self {
        Self::Step { hash: s.hash }
    }
}

impl From<&Step> for Dependency {
    fn from(s: &Step) -> Self {
        Self::Step {
            hash: s.hash.to_string(),
        }
    }
}
