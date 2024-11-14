use anyhow::Result;
use git2::{Repository, BranchType};
use std::collections::HashMap;

fn collect_branches(repo: &Repository, branch_type: BranchType) -> Result<HashMap<String, git2::Oid>> {
    let mut branches = HashMap::new();
    for branch_result in repo.branches(Some(branch_type))? {
        let (branch, _) = branch_result?;
        let name = branch.name()?.unwrap_or("").to_string();
        if let Some(commit) = branch.get().target() {
            branches.insert(name, commit);
        }
    }
    Ok(branches)
}

pub fn generate_branch_graph(include_remote: bool) -> Result<()> {
    let repo = Repository::open(".")?;
    let mut branches = collect_branches(&repo, BranchType::Local)?;

    if include_remote {
        branches.extend(collect_branches(&repo, BranchType::Remote)?);
    }

    // Generate and print the graph
    print_branch_graph(&repo, &branches)?;

    Ok(())
}

fn print_branch_graph(repo: &Repository, branches: &HashMap<String, git2::Oid>) -> Result<()> {
    // Implementation of branch graph visualization
    // This is a simplified version - you could make it more sophisticated
    println!("\nBranch Relationships:");
    println!("===================");

    for (branch_name, commit_id) in branches {
        let commit = repo.find_commit(*commit_id)?;
        let parent_count = commit.parent_count();

        print!("{} ", branch_name);
        if parent_count > 0 {
            print!("← ");
            let parent = commit.parent(0)?;
            for (parent_branch, parent_commit) in branches {
                if parent.id() == *parent_commit {
                    println!("{}", parent_branch);
                    break;
                }
            }
        } else {
            println!("(root)");
        }
    }

    Ok(())
}