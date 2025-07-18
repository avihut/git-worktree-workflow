use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

pub struct GitCommand {
    quiet: bool,
}

impl GitCommand {
    pub fn new(quiet: bool) -> Self {
        Self { quiet }
    }

    pub fn clone_bare(&self, repo_url: &str, target_dir: &Path) -> Result<()> {
        let mut cmd = Command::new("git");
        cmd.args(["clone", "--bare"]);

        if self.quiet {
            cmd.arg("--quiet");
        }

        cmd.arg(repo_url).arg(target_dir);

        let output = cmd
            .output()
            .context("Failed to execute git clone command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Git clone failed: {}", stderr);
        }

        Ok(())
    }

    pub fn init_bare(&self, target_dir: &Path, initial_branch: &str) -> Result<()> {
        let mut cmd = Command::new("git");
        cmd.args(["init", "--bare"]);

        if self.quiet {
            cmd.arg("--quiet");
        }

        cmd.arg(format!("--initial-branch={initial_branch}"))
            .arg(target_dir);

        let output = cmd.output().context("Failed to execute git init command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Git init failed: {}", stderr);
        }

        Ok(())
    }

    pub fn worktree_add(&self, path: &Path, branch: &str) -> Result<()> {
        let mut cmd = Command::new("git");
        cmd.args(["worktree", "add"]);

        if self.quiet {
            cmd.arg("--quiet");
        }

        cmd.arg(path).arg(branch);

        let output = cmd
            .output()
            .context("Failed to execute git worktree add command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Git worktree add failed: {}", stderr);
        }

        Ok(())
    }

    pub fn worktree_add_orphan(&self, path: &Path, branch_name: &str) -> Result<()> {
        let mut cmd = Command::new("git");
        cmd.args(["worktree", "add"]);

        if self.quiet {
            cmd.arg("--quiet");
        }

        // Explicitly specify the branch name to avoid Git's path-based inference
        cmd.arg(path).arg("-b").arg(branch_name);

        let output = cmd
            .output()
            .context("Failed to execute git worktree add command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Git worktree add failed: {}", stderr);
        }

        Ok(())
    }

    pub fn worktree_add_new_branch(
        &self,
        path: &Path,
        new_branch: &str,
        base_branch: &str,
    ) -> Result<()> {
        let mut cmd = Command::new("git");
        cmd.args(["worktree", "add"]);

        if self.quiet {
            cmd.arg("--quiet");
        }

        cmd.arg(path).arg("-b").arg(new_branch).arg(base_branch);

        let output = cmd
            .output()
            .context("Failed to execute git worktree add command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Git worktree add failed: {}", stderr);
        }

        Ok(())
    }

    pub fn worktree_remove(&self, path: &Path, force: bool) -> Result<()> {
        let mut cmd = Command::new("git");
        cmd.args(["worktree", "remove"]);

        if force {
            cmd.arg("--force");
        }

        cmd.arg(path);

        let output = cmd
            .output()
            .context("Failed to execute git worktree remove command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Git worktree remove failed: {}", stderr);
        }

        Ok(())
    }

    pub fn worktree_list_porcelain(&self) -> Result<String> {
        let output = Command::new("git")
            .args(["worktree", "list", "--porcelain"])
            .output()
            .context("Failed to execute git worktree list command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Git worktree list failed: {}", stderr);
        }

        String::from_utf8(output.stdout).context("Failed to parse git worktree list output")
    }

    pub fn fetch(&self, remote: &str, prune: bool) -> Result<()> {
        let mut cmd = Command::new("git");
        cmd.args(["fetch", remote]);

        if prune {
            cmd.arg("--prune");
        }

        let output = cmd
            .output()
            .context("Failed to execute git fetch command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Git fetch failed: {}", stderr);
        }

        Ok(())
    }

    pub fn push_set_upstream(&self, remote: &str, branch: &str) -> Result<()> {
        let output = Command::new("git")
            .args(["push", "--set-upstream", remote, branch])
            .output()
            .context("Failed to execute git push command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Git push failed: {}", stderr);
        }

        Ok(())
    }

    pub fn set_upstream(&self, remote: &str, branch: &str) -> Result<()> {
        let upstream = format!("{remote}/{branch}");
        let output = Command::new("git")
            .args(["branch", &format!("--set-upstream-to={upstream}")])
            .output()
            .context("Failed to execute git branch command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Git set upstream failed: {}", stderr);
        }

        Ok(())
    }

    pub fn branch_delete(&self, branch: &str, force: bool) -> Result<()> {
        let mut cmd = Command::new("git");
        cmd.args(["branch"]);

        if force {
            cmd.arg("-D");
        } else {
            cmd.arg("-d");
        }

        cmd.arg(branch);

        let output = cmd
            .output()
            .context("Failed to execute git branch command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Git branch delete failed: {}", stderr);
        }

        Ok(())
    }

    pub fn branch_list_verbose(&self) -> Result<String> {
        let output = Command::new("git")
            .args(["branch", "-vv"])
            .output()
            .context("Failed to execute git branch command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Git branch list failed: {}", stderr);
        }

        String::from_utf8(output.stdout).context("Failed to parse git branch output")
    }

    pub fn for_each_ref(&self, format: &str, refs: &str) -> Result<String> {
        let output = Command::new("git")
            .args(["for-each-ref", &format!("--format={format}"), refs])
            .output()
            .context("Failed to execute git for-each-ref command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Git for-each-ref failed: {}", stderr);
        }

        String::from_utf8(output.stdout).context("Failed to parse git for-each-ref output")
    }

    pub fn show_ref_exists(&self, ref_name: &str) -> Result<bool> {
        let output = Command::new("git")
            .args(["show-ref", "--verify", "--quiet", ref_name])
            .output()
            .context("Failed to execute git show-ref command")?;

        Ok(output.status.success())
    }

    pub fn ls_remote_heads(&self, remote: &str, branch: Option<&str>) -> Result<String> {
        let mut cmd = Command::new("git");
        cmd.args(["ls-remote", "--heads", remote]);

        if let Some(branch) = branch {
            cmd.arg(format!("refs/heads/{branch}"));
        }

        let output = cmd
            .output()
            .context("Failed to execute git ls-remote command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Git ls-remote failed: {}", stderr);
        }

        String::from_utf8(output.stdout).context("Failed to parse git ls-remote output")
    }

    pub fn get_git_dir(&self) -> Result<String> {
        let output = Command::new("git")
            .args(["rev-parse", "--git-dir"])
            .output()
            .context("Failed to execute git rev-parse command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Git rev-parse failed: {}", stderr);
        }

        String::from_utf8(output.stdout)
            .context("Failed to parse git rev-parse output")
            .map(|s| s.trim().to_string())
    }

    pub fn remote_set_head_auto(&self, remote: &str) -> Result<()> {
        let output = Command::new("git")
            .args(["remote", "set-head", remote, "--auto"])
            .output()
            .context("Failed to execute git remote set-head command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Git remote set-head failed: {}", stderr);
        }

        Ok(())
    }

    pub fn fetch_refspec(&self, remote: &str, refspec: &str) -> Result<()> {
        let mut cmd = Command::new("git");
        cmd.args(["fetch", remote, refspec]);

        if self.quiet {
            cmd.arg("--quiet");
        }

        let output = cmd
            .output()
            .context("Failed to execute git fetch command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Git fetch with refspec failed: {}", stderr);
        }

        Ok(())
    }

    pub fn rev_list_count(&self, range: &str) -> Result<u32> {
        let output = Command::new("git")
            .args(["rev-list", "--count", range])
            .output()
            .context("Failed to execute git rev-list command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Git rev-list failed: {}", stderr);
        }

        let stdout =
            String::from_utf8(output.stdout).context("Failed to parse git rev-list output")?;

        stdout
            .trim()
            .parse::<u32>()
            .context("Failed to parse commit count as number")
    }

    /// Check if current directory is inside a Git work tree
    pub fn rev_parse_is_inside_work_tree(&self) -> Result<bool> {
        let output = Command::new("git")
            .args(["rev-parse", "--is-inside-work-tree"])
            .output()
            .context("Failed to execute git rev-parse command")?;

        // Git rev-parse --is-inside-work-tree returns exit code 0 when inside work tree
        Ok(output.status.success())
    }

    /// Get the Git common directory path
    pub fn rev_parse_git_common_dir(&self) -> Result<String> {
        let output = Command::new("git")
            .args(["rev-parse", "--git-common-dir"])
            .output()
            .context("Failed to execute git rev-parse command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Git rev-parse failed: {}", stderr);
        }

        String::from_utf8(output.stdout)
            .context("Failed to parse git rev-parse output")
            .map(|s| s.trim().to_string())
    }

    /// Get the short name of the current branch
    pub fn symbolic_ref_short_head(&self) -> Result<String> {
        let output = Command::new("git")
            .args(["symbolic-ref", "--short", "HEAD"])
            .output()
            .context("Failed to execute git symbolic-ref command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Git symbolic-ref failed: {}", stderr);
        }

        String::from_utf8(output.stdout)
            .context("Failed to parse git symbolic-ref output")
            .map(|s| s.trim().to_string())
    }

    /// Execute git ls-remote with symref to get remote HEAD
    pub fn ls_remote_symref(&self, remote_url: &str) -> Result<String> {
        let output = Command::new("git")
            .args(["ls-remote", "--symref", remote_url, "HEAD"])
            .output()
            .context("Failed to execute git ls-remote command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Git ls-remote failed: {}", stderr);
        }

        String::from_utf8(output.stdout).context("Failed to parse git ls-remote output")
    }

    /// Check if specific remote branch exists
    pub fn ls_remote_branch_exists(&self, remote_name: &str, branch: &str) -> Result<bool> {
        let output = Command::new("git")
            .args([
                "ls-remote",
                "--heads",
                remote_name,
                &format!("refs/heads/{branch}"),
            ])
            .output()
            .context("Failed to execute git ls-remote command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Git ls-remote failed: {}", stderr);
        }

        let stdout =
            String::from_utf8(output.stdout).context("Failed to parse git ls-remote output")?;
        Ok(!stdout.trim().is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_command_new() {
        let git = GitCommand::new(true);
        assert!(git.quiet);

        let git = GitCommand::new(false);
        assert!(!git.quiet);
    }
}
