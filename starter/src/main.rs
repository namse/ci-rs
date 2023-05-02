use ci_rs_server_shared::{
    ci_rs_core::{Dependency, Step},
    queue, serde_yaml, step_storage,
};
use std::process::Command;

fn main() {
    let output = Command::new("cargo")
        .args(["run"])
        .current_dir("../example")
        .output()
        .unwrap();
    let input = String::from_utf8(output.stdout).unwrap();

    let steps: Vec<Step> = serde_yaml::from_str(&input).unwrap();

    println!("steps: {:#?}", steps);

    step_storage::save_steps(&steps);

    let starting_step_hashs: Vec<_> = steps
        .iter()
        .filter_map(|step| {
            step.dependencies
                .iter()
                .all(|dep| match dep {
                    Dependency::String(_) => true,
                    Dependency::Step { .. } => false,
                })
                .then(|| &step.hash)
        })
        .collect();

    println!("starting_steps: {:#?}", starting_step_hashs);

    queue::start_steps(starting_step_hashs);
}
