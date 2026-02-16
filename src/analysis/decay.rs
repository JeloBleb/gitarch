use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    analysis::ownership::{
        get_file_concentrations, get_owners, get_primary_owners, get_user_last_active,
    },
    repo::CommitInfo,
};

const DECAY_THRESHOLD: i64 = 180 * 24 * 60 * 60;
const STALENESS_WEIGHT: f64 = 0.3;
const INACTIVITY_WEIGHT: f64 = 0.7;

pub fn get_decay(commits: &[CommitInfo]) -> HashMap<String, f64> {
    let mut file_decays: HashMap<String, f64> = HashMap::new();

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

        let staleness = ((time - last_modified) as f64 / DECAY_THRESHOLD as f64).min(1.0);
        let inactivity = ((time - user_last_active) as f64 / DECAY_THRESHOLD as f64).min(1.0);
        let decay = (STALENESS_WEIGHT * staleness + INACTIVITY_WEIGHT * inactivity * concentration)
            .min(1.0);

        file_decays.insert(path.clone(), decay);
    }

    file_decays
}

pub fn get_revision_counts(commits: &[CommitInfo]) -> HashMap<String, usize> {
    let mut revision_counts: HashMap<String, usize> = HashMap::new();

    for commit in commits {
        commit
            .file_changes
            .iter()
            .for_each(|p| *revision_counts.entry(p.path.clone()).or_insert(0) += 1);
    }

    revision_counts
}

pub fn get_line_changes(commits: &[CommitInfo]) -> HashMap<String, (usize, usize)> {
    let mut line_changes: HashMap<String, (usize, usize)> = HashMap::new();

    for commit in commits {
        for file in &commit.file_changes {
            let (insertions, deletions) = line_changes.entry(file.path.clone()).or_insert((0, 0));
            *insertions += file.insertions;
            *deletions += file.deletions;
        }
    }

    line_changes
}

pub fn get_files_last_modified(commits: &[CommitInfo]) -> HashMap<String, i64> {
    let mut timestamps: HashMap<String, i64> = HashMap::new();

    for commit in commits {
        for file in &commit.file_changes {
            timestamps
                .entry(file.path.clone())
                .or_insert(commit.timestamp);
        }
    }

    timestamps
}
