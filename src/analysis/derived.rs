use crate::repo::CommitInfo;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::analysis::metrics::*;

const DAYS_TO_SECONDS: i64 = 24 * 60 * 60;

pub fn get_decay(commits: &[CommitInfo], decay_threshold: i64) -> HashMap<String, f64> {
    let mut file_decays: HashMap<String, f64> = HashMap::new();
    let decay_threshold = decay_threshold * DAYS_TO_SECONDS;

    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let files_last_modified = get_files_last_modified(commits);
    let file_owners = get_owners(commits);
    let file_primary_owners = get_primary_owners(&file_owners);
    let file_concentrations = get_file_concentrations(&file_owners, &file_primary_owners);
    let users_last_active = get_user_last_active(commits);

    for path in file_owners.keys() {
        let last_modified = files_last_modified.get(path).unwrap();
        let primary_owner = file_primary_owners.get(path).unwrap();
        let concentration = file_concentrations.get(path).unwrap();
        let user_last_active = users_last_active.get(primary_owner).unwrap();

        let staleness = ((time - last_modified) as f64 / decay_threshold as f64).min(1.0);
        let inactivity = ((time - user_last_active) as f64 / decay_threshold as f64).min(1.0);
        let decay = (0.7 * staleness + 0.3 * inactivity * concentration).min(1.0);

        file_decays.insert(path.clone(), decay);
    }

    file_decays
}

pub fn get_file_concentrations(
    file_owners: &HashMap<String, HashMap<String, usize>>,
    file_primary_owners: &HashMap<String, String>,
) -> HashMap<String, f64> {
    let mut files: HashMap<String, f64> = HashMap::new();

    for (path, authors) in file_owners {
        let concentration = *authors
            .get(
                file_primary_owners
                    .get(path)
                    .expect("primary owner invalid"),
            )
            .unwrap() as f64
            / authors.values().sum::<usize>() as f64;

        files.insert(path.clone(), concentration);
    }

    files
}
