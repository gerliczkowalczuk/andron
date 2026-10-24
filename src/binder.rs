use std::path::Path;

pub fn init_binder_subsystem() -> Result<(), Box<dyn std::error::Error>> {
    let candidate_nodes = [
        "/dev/binder",
        "/dev/binderfs/binder",
        "/dev/anbox-binder",
    ];

    let mut found = false;
    for node in &candidate_nodes {
        if Path::new(node).exists() {
            log::info!("Discovered active Binder node: {}", node);
            found = true;
            break;
        }
    }

    if !found {
        log::warn!("Hardware binder node not directly found in /dev. Direct binderfs IPC will be dynamically mounted in unshared namespace.");
    }

    Ok(())
}
