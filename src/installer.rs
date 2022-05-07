use anyhow::{Context, Result};
use std::path::PathBuf;

use crate::config::{Config, Package};
use crate::PKG_NAME;

#[derive(Debug)]
pub struct Installer {
    config: Config,
    pack_dir: PathBuf,
    upgrade_during_install: bool,
}

impl Installer {
    pub fn new(config: Config) -> Self {
        let pack_dir = home::home_dir()
            .map(|d| d.join(".local/share/nvim/site/pack"))
            .unwrap();

        Self {
            config,
            pack_dir,
            upgrade_during_install: false,
        }
    }
    pub fn set_upgrade_during_install(&mut self, b: bool) {
        self.upgrade_during_install = b;
    }
    pub fn clone_repo(&self, remote_path: &str, cfg: &Package) -> Result<()> {
        let repo_path = self.pack_dir.join(PKG_NAME).join("start").join(
            cfg.rename
                .as_ref()
                .unwrap_or(&remote_path.split('/').last().unwrap().to_string()),
        );
        match git2::Repository::open(&repo_path) {
            Err(e) if e.code() == git2::ErrorCode::NotFound => {}
            Err(e) => return Err(e.into()),
            Ok(_) => {
                if self.upgrade_during_install {
                    self.pull_repo(remote_path, cfg)?;
                }
                return Ok(());
            }
        }

        if let Some(rename_to) = &cfg.rename {
            println!("Cloning {} to {}", &remote_path, &rename_to);
        } else {
            println!("Cloning {}", &remote_path);
        }

        let remote_url = format!("https://github.com/{}", remote_path);
        git2::build::RepoBuilder::new()
            .clone(&remote_url, &repo_path)
            .context("failed to clone repository")?;
        println!("Cloned {}", &remote_path);

        Ok(())
    }

    pub fn pull_repo(&self, remote_path: &str, cfg: &Package) -> Result<()> {
        let repo_path = self.pack_dir.join(PKG_NAME).join("start").join(
            cfg.rename
                .as_ref()
                .unwrap_or(&remote_path.split('/').last().unwrap().to_string()),
        );
        let repo = match git2::Repository::open(&repo_path) {
            Err(e) if e.code() == git2::ErrorCode::NotFound => return Ok(()),
            Err(e) => return Err(e.into()),
            Ok(repo) => repo,
        };

        let mut remote = repo.find_remote("origin")?;

        println!("Updating {}", &remote_path);
        for branch in repo.branches(None)? {
            let (branch, branch_type) = branch?;

            if let git2::BranchType::Local = branch_type {
                let branch_name = branch.name()?.unwrap();

                remote.fetch(&[&branch_name], None, None)?;
                let fetch_head_ref = repo.find_reference("FETCH_HEAD")?;
                let fetch_commit = repo.reference_to_annotated_commit(&fetch_head_ref)?;

                let mut branch_head_ref =
                    repo.find_reference(&format!("refs/heads/{}", &branch_name))?;
                let (analysis, _pref) =
                    repo.merge_analysis_for_ref(&branch_head_ref, &[&fetch_commit])?;

                if analysis.is_fast_forward() {
                    println!("Fast forwarding {}...", &branch_name);
                    branch_head_ref.set_target(fetch_commit.id(), "fast forwarding")?;
                    println!("Fast forwarded {} to {}", &branch_name, fetch_commit.id());
                    repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
                    println!("Updated {}", &remote_path);
                } else if analysis.is_up_to_date() {
                    println!("{} Already up to date", &remote_path);
                } else {
                    unimplemented!()
                }
            }
        }

        Ok(())
    }

    pub fn all_repos<F>(&self, f: F) -> Result<()>
    where
        F: Fn(&Self, &str, &Package) -> Result<()>,
    {
        for (remote_path, pkg) in &self.config.packages {
            f(self, remote_path, pkg)?;
        }

        Ok(())
    }
}
