use crate::{config::Config, error::BranchError};
use anyhow::Result;
use chrono::{DateTime, Local, TimeZone};
use colored::*;
use git2::{BranchType, Repository};
use indicatif::{ProgressBar, ProgressStyle};

pub fn list_branches(config: &Config, sort_by_date: bool) -> Result<()> {
    let repo = Repository::open(".")?;
    let mut branches = get_branch_info(&repo)?;

    if sort_by_date {
        branches.sort_by(|a, b| b.last_commit_date.cmp(&a.last_commit_date));
    }

    println!("\n{}", "Branch Information:".bold());
    println!("{}", "=================".bold());

    for branch_info in branches {
        let age = Local::now()
            .signed_duration_since(branch_info.last_commit_date)
            .num_days();

        let branch_name = if config.protected_branches.contains(&branch_info.name) {
            branch_info.name.green()
        } else {
            branch_info.name.normal()
        };

        println!(
            "{} (last commit: {} days ago by {})",
            branch_name,
            age,
            branch_info.last_author.cyan()
        );
    }

    Ok(())
}

pub fn cleanup_stale_branches(config: &Config, days: u32, dry_run: bool) -> Result<()> {
    let repo = Repository::open(".")?;
    let branches = get_branch_info(&repo)?;
    let now = Local::now();

    let mut stale_branches = Vec::new();

    for branch_info in branches {
        if config.protected_branches.contains(&branch_info.name) {
            continue;
        }

        let age = now
            .signed_duration_since(branch_info.last_commit_date)
            .num_days();

        if age > days as i64 {
            stale_branches.push(branch_info);
        }
    }

    if stale_branches.is_empty() {
        println!("No stale branches found!");
        return Ok(());
    }

    println!("\nFound {} stale branches:", stale_branches.len());
    for branch in &stale_branches {
        println!("  - {}", branch.name);
    }

    if !dry_run {
        let confirm = dialoguer::Confirm::new()
            .with_prompt("Do you want to delete these branches?")
            .interact()?;

        if confirm {
            let pb = ProgressBar::new(stale_branches.len() as u64);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")?,
            );

            for branch in stale_branches {
                let mut git_branch = repo.find_branch(&branch.name, BranchType::Local)?;
                git_branch.delete()?;
                pb.inc(1);
            }

            pb.finish_with_message("Branches deleted successfully!");
        }
    }

    Ok(())
}

#[derive(Debug)]
struct BranchInfo {
    name: String,
    last_commit_date: DateTime<Local>,
    last_author: String,
}

fn get_branch_info(repo: &Repository) -> Result<Vec<BranchInfo>> {
    let mut branch_info = Vec::new();
    let branches = repo.branches(Some(BranchType::Local))?;

    for branch_result in branches {
        let (branch, _) = branch_result?;
        let name = branch.name()?.ok_or_else(|| {
            BranchError::OperationError("Invalid branch name".to_string())
        })?.to_string();

        if let Some(commit) = branch.get().target() {
            let commit = repo.find_commit(commit)?;
            let time = commit.time();
            let date = Local.timestamp_opt(time.seconds(), 0).unwrap();
            let author = commit.author().name().unwrap_or("Unknown").to_string();

            branch_info.push(BranchInfo {
                name,
                last_commit_date: date,
                last_author: author,
            });
        }
    }

    Ok(branch_info)
}