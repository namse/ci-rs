use ci_rs_server_shared::{ci_rs_core::Step, layer_storage, step_storage};

fn main() {
    let step_hash = "";
    let next_steps = vec![];

    let step: Step = step_storage::get_step(&step_hash);

    if is_step_already_done(&step_hash) {
        return;
    }

    if !lock_step(&step_hash) {
        return;
    }

    let image = create_image(&step);

    image.run_image(&step.commands);
    let layer = image.get_last_layer();

    layer_storage::save_layer(&step.hash, &layer);

    queue::start_steps(next_steps);
}
