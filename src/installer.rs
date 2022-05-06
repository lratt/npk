use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

use crate::config::{Config, PackageAuthor};
use crate::PKG_NAME;

#[derive(Debug)]
pub struct Installer {
    config: Config,
    pack_dir: PathBuf,
}

impl Installer {
    pub fn new(config: Config) -> Self {
        let pack_dir = home::home_dir()
            .map(|d| d.join(".local/share/nvim/site/pack"))
            .unwrap();

        Self { config, pack_dir }
    }
    pub fn clone_repo(
        &self,
        author: &str,
        (repo_name, cfg): (&String, &PackageAuthor),
    ) -> Result<()> {
        let repo_path = self.pack_dir.join(PKG_NAME).join("start").join(repo_name);
        match git2::Repository::open(&repo_path) {
            Err(e) if e.code() == git2::ErrorCode::NotFound => {}
            Err(e) => return Err(e.into()),
            Ok(_) => return Ok(()),
        }

        let repo_url = cfg
            .repo
            .clone()
            .unwrap_or_else(|| format!("https://github.com/{}/{}", author, repo_name));

        println!("Cloning {}", &repo_url);
        git2::build::RepoBuilder::new()
            .clone(&repo_url, &repo_path)
            .context("failed to clone repository")?;
        println!("Cloned {}", &repo_url);

        Ok(())
    }

    pub fn pull_repo(
        &self,
        author: &str,
        (repo_name, cfg): (&String, &PackageAuthor),
    ) -> Result<()> {
        let repo_path = self.pack_dir.join(PKG_NAME).join("start").join(repo_name);
        let repo = match git2::Repository::open(&repo_path) {
            Err(e) if e.code() == git2::ErrorCode::NotFound => return Ok(()),
            Err(e) => return Err(e.into()),
            Ok(repo) => repo,
        };

        let mut remote = repo.find_remote("origin")?;


        println!("Updating {}/{}", &author, &repo_name);
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
                    println!("Updated {}/{}", &author, &repo_name);
                } else if analysis.is_up_to_date() {
                    println!("{}/{} Already up to date", &author, &repo_name);
                } else {
                    unimplemented!()
                }
            }
        }

        Ok(())
    }

    pub fn install(&self) -> Result<()> {
        for (author, pkgs) in &self.config.packages {
            for pkg in pkgs {
                self.clone_repo(author, pkg)?;
            }
        }

        Ok(())
    }
    pub fn upgrade(&self) -> Result<()> {
        for (author, pkgs) in &self.config.packages {
            for pkg in pkgs {
                self.pull_repo(author, pkg)?;
            }
        }

        Ok(())
    }
}
