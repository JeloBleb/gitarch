use crate::analysis::{derived::*, metrics::*};
use crate::*;
use cliux::Table;
use csv::*;
use itertools::Itertools;
use serde::Serialize;
use serde_json::to_string_pretty;

#[derive(Serialize)]
struct DecayEntry {
    file: String,
    score: f64,
}

#[derive(Serialize)]
struct CouplingEntry {
    file_a: String,
    file_b: String,
    percent: f64,
}

#[derive(Serialize)]
struct OwnershipEntry {
    file: String,
    owner: String,
}

#[derive(Serialize)]
struct CommunicationEntry {
    author_a: String,
    author_b: String,
    count: usize,
}

#[derive(Serialize)]
struct ChurnEntry {
    file: String,
    revisions: usize,
    insertions: usize,
    deletions: usize,
    created: NaiveDate,
    last_modified: NaiveDate,
}
pub fn print_summary(commits: &[CommitInfo], config: OutputConfig) -> anyhow::Result<()> {
    let summary = get_summary(commits);

    match config.format {
        Format::Json => {
            let json = to_string_pretty(&summary).unwrap();
            println!("{json}");
        }
        Format::Table => {
            let SummaryStats {
                commits,
                files,
                file_changes,
                authors,
            } = summary;

            let table = Table::new()
                .headers(&["Commits", "Files", "File Changes", "Authors"])
                .row(&[
                    &commits.to_string(),
                    &files.to_string(),
                    &file_changes.to_string(),
                    &authors.to_string(),
                ]);
            table.print();
        }
        Format::Csv => {
            let mut wtr = Writer::from_writer(std::io::stdout());
            wtr.serialize(summary)?;
            wtr.flush()?;
        }
    }

    Ok(())
}

pub fn print_owner_summary(commits: &[CommitInfo], config: OutputConfig) -> anyhow::Result<()> {
    let owner_summaries = get_owner_summary(commits);

    match config.format {
        Format::Json => {
            let json = to_string_pretty(&owner_summaries).unwrap();
            println!("{json}");
        }
        Format::Table => {
            let mut table = Table::new().headers(&[
                "Author",
                "Files Touched",
                "File Changes",
                "Line Revisions",
            ]);
            for (name, stats) in owner_summaries {
                table = table.row(&[
                    &name,
                    &stats.files.len().to_string(),
                    &stats.file_changes.to_string(),
                    &stats.revisions.to_string(),
                ]);
            }
            table.print();
        }
        Format::Csv => {
            let mut wtr = Writer::from_writer(std::io::stdout());
            wtr.write_record(["author", "files_touched", "file_changes", "revisions"])?;
            for (name, stats) in owner_summaries {
                wtr.write_record([
                    &name,
                    &stats.files.len().to_string(),
                    &stats.file_changes.to_string(),
                    &stats.revisions.to_string(),
                ])?;
            }
            wtr.flush()?;
        }
    }

    Ok(())
}

pub fn print_decay(
    commits: &[CommitInfo],
    decay_threshold: i64,
    config: OutputConfig,
) -> anyhow::Result<()> {
    let decay = get_decay(commits, decay_threshold);
    let decay = filter_deleted(decay, commits);
    let decay = decay
        .into_iter()
        .map(|(file, score)| DecayEntry { file, score })
        .sorted_by(|a, b| b.score.total_cmp(&a.score))
        .take(config.top.unwrap_or(usize::MAX));

    match config.format {
        Format::Json => {
            let json = to_string_pretty(&decay.collect::<Vec<DecayEntry>>()).unwrap();
            println!("{json}");
        }
        Format::Table => {
            let mut table = Table::new().headers(&["File", "Decay Score"]);
            for DecayEntry { file, score } in decay {
                let score = (score * 100.0).round() / 100.0;
                table = table.row(&[&file, &score.to_string()]);
            }
            table.print();
        }
        Format::Csv => {
            let mut wtr = Writer::from_writer(std::io::stdout());
            for entry in decay {
                wtr.serialize(entry)?;
            }
            wtr.flush()?;
        }
    }

    Ok(())
}

pub fn print_coupling(
    commits: &[CommitInfo],
    max_changeset_size: usize,
    min_coupling_percent: usize,
    min_revision_count: usize,
    config: OutputConfig,
) -> anyhow::Result<()> {
    let coupling = get_coupling_percentage(commits, max_changeset_size, min_revision_count);

    let file_statuses = get_file_statuses(commits);

    let coupling = coupling
        .into_iter()
        .filter(|p| p.1 > min_coupling_percent as f64 / 100.0)
        .filter(|p| {
            file_statuses.get(&p.0.0) != Some(&FileStatus::Deleted)
                && file_statuses.get(&p.0.1) != Some(&FileStatus::Deleted)
        })
        .sorted_by(|a, b| b.1.total_cmp(&a.1))
        .map(|((file_a, file_b), percent)| CouplingEntry {
            file_a,
            file_b,
            percent,
        })
        .take(config.top.unwrap_or(usize::MAX));

    match config.format {
        Format::Json => {
            let json =
                serde_json::to_string_pretty(&coupling.collect::<Vec<CouplingEntry>>()).unwrap();
            println!("{json}");
        }
        Format::Table => {
            let mut table = Table::new().headers(&["File Pair", "Coupling"]);
            for CouplingEntry {
                file_a,
                file_b,
                percent,
            } in coupling
            {
                table = table.row(&[
                    &format!("{} and {}", file_a, file_b),
                    &format!("{}%", (percent * 100.0).round()),
                ])
            }
            table.print();
        }
        Format::Csv => {
            let mut wtr = Writer::from_writer(std::io::stdout());
            for entry in coupling {
                wtr.serialize(entry)?;
            }
            wtr.flush()?;
        }
    }

    Ok(())
}

pub fn print_owners(commits: &[CommitInfo], config: OutputConfig) -> anyhow::Result<()> {
    let owners = get_primary_owners(&get_owners(commits));
    let owners = filter_deleted(owners, commits)
        .into_iter()
        .sorted_by(|a, b| a.0.cmp(&b.0))
        .map(|(file, owner)| OwnershipEntry { file, owner })
        .take(config.top.unwrap_or(usize::MAX));

    match config.format {
        Format::Json => {
            let json = to_string_pretty(&owners.collect::<Vec<OwnershipEntry>>()).unwrap();
            println!("{json}");
        }
        Format::Table => {
            let mut table = Table::new().headers(&["File", "Owner"]);
            for OwnershipEntry { file, owner } in owners {
                table = table.row(&[&file, &owner]);
            }
            table.print();
        }
        Format::Csv => {
            let mut wtr = Writer::from_writer(std::io::stdout());
            for entry in owners {
                wtr.serialize(entry)?;
            }
            wtr.flush()?;
        }
    }

    Ok(())
}

pub fn print_communication(commits: &[CommitInfo], config: OutputConfig) -> anyhow::Result<()> {
    let owner_coupling = get_owner_coupling(commits);
    let owner_coupling = owner_coupling
        .into_iter()
        .sorted_by(|(_, coupling1), (_, coupling2)| coupling2.cmp(coupling1))
        .map(|((author_a, author_b), count)| CommunicationEntry {
            author_a,
            author_b,
            count,
        })
        .take(config.top.unwrap_or(usize::MAX));

    match config.format {
        Format::Json => {
            let json =
                to_string_pretty(&owner_coupling.collect::<Vec<CommunicationEntry>>()).unwrap();
            println!("{json}");
        }
        Format::Table => {
            let mut table = Table::new().headers(&["Owner Pair", "File Overlap"]);
            for CommunicationEntry {
                author_a,
                author_b,
                count,
            } in owner_coupling
            {
                table = table.row(&[
                    &format!("{} and {}", author_a, author_b),
                    &count.to_string(),
                ]);
            }
            table.print();
        }
        Format::Csv => {
            let mut wtr = Writer::from_writer(std::io::stdout());
            for entry in owner_coupling {
                wtr.serialize(entry)?;
            }
            wtr.flush()?;
        }
    }

    Ok(())
}

pub fn print_churn(
    commits: &[CommitInfo],
    filtered_commits: &[CommitInfo],
    config: OutputConfig,
) -> anyhow::Result<()> {
    let last_modified = get_files_last_modified(commits);
    let created = get_files_creation(commits);

    let line_changes = get_line_changes(filtered_commits);
    let line_changes = filter_deleted(line_changes, commits)
        .into_iter()
        .sorted_by(|(file, _), (file2, _)| file.cmp(file2))
        .take(config.top.unwrap_or(usize::MAX));
    let revisions = get_revision_counts(filtered_commits);

    let mut churn_entries: Vec<ChurnEntry> = Vec::new();

    for (file, (insertions, deletions)) in line_changes {
        let revisions = *revisions.get(&file).unwrap();
        let created = DateTime::from_timestamp(*created.get(&file).unwrap(), 0)
            .unwrap()
            .date_naive();
        let last_modified = DateTime::from_timestamp(*last_modified.get(&file).unwrap(), 0)
            .unwrap()
            .date_naive();
        churn_entries.push(ChurnEntry {
            file,
            revisions,
            insertions,
            deletions,
            created,
            last_modified,
        });
    }

    match config.format {
        Format::Json => {
            let json = to_string_pretty(&churn_entries).unwrap();
            println!("{json}");
        }
        Format::Table => {
            let mut table = Table::new().headers(&[
                "File",
                "Revisions",
                "Insertions",
                "Deletions",
                "Created",
                "Last Modified",
            ]);
            for ChurnEntry {
                file,
                revisions,
                insertions,
                deletions,
                created,
                last_modified,
            } in churn_entries
            {
                table = table.row(&[
                    &file,
                    &revisions.to_string(),
                    &insertions.to_string(),
                    &deletions.to_string(),
                    &created.to_string(),
                    &last_modified.to_string(),
                ])
            }
            table.print();
        }
        Format::Csv => {
            let mut wtr = Writer::from_writer(std::io::stdout());
            for entry in churn_entries {
                wtr.serialize(entry)?;
            }
            wtr.flush()?;
        }
    }

    Ok(())
}
