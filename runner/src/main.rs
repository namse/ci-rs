use ci_rs_core::Step;
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

    let starting_step_nodes = {
        let mut starting_step_nodes = vec![];

        for step in &steps {
            if !step.dependencies.iter().all(|dep| match dep {
                ci_rs_core::Dependency::String(_) => true,
                ci_rs_core::Dependency::Step { .. } => false,
            }) {
                continue;
            }

            starting_step_nodes.push(StepFlowNode {
                step,
                nexts: get_nexts(step, &steps),
            });
        }

        fn get_nexts<'a>(step: &'a Step, steps: &'a [Step]) -> Vec<StepFlowNode<'a>> {
            let mut nexts = vec![];

            for other_step in steps {
                if std::ptr::eq(step, other_step) {
                    continue;
                }
                if other_step.dependencies.iter().any(|dep| match dep {
                    ci_rs_core::Dependency::String(_) => false,
                    ci_rs_core::Dependency::Step { hash } => hash == &step.hash,
                }) {
                    nexts.push(StepFlowNode {
                        step: other_step,
                        nexts: get_nexts(other_step, steps),
                    });
                }
            }

            nexts
        }

        starting_step_nodes
    };

    println!("starting_step_nodes: {:#?}", starting_step_nodes);
}

#[derive(Debug)]
struct StepFlowNode<'a> {
    step: &'a Step,
    nexts: Vec<StepFlowNode<'a>>,
}
