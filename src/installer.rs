use anyhow::{Context, Result};
use std::path::Path;

use crate::config::{Config, PackageAuthorConfig, PackageConfig};

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Debug)]
pub struct Installer {
    config: Config,
}

impl Installer {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
    pub fn clone_repos(&self) -> Result<()> {
        let data_dir = home::home_dir()
            .map(|d| d.join(".local/share/nvim/site/pack"))
            .unwrap();

        for (author, pkgs) in &self.config.packages {
            for pkg in pkgs {
                self.clone_repo(author, pkg, &data_dir)?;
            }
        }

        Ok(())
    }
    pub fn clone_repo<P: AsRef<Path>>(
        &self,
        author: &str,
        (repo, cfg): (&String, &PackageAuthorConfig),
        into: P,
    ) -> Result<()> {
        let repo_path = into.as_ref().join(PKG_NAME).join("start").join(repo);
        match git2::Repository::open(&repo_path) {
            Err(e) if e.code() == git2::ErrorCode::NotFound => {}
            Err(e) => return Err(e.into()),
            Ok(_) => return Ok(()),
        }

        let repo_url = cfg
            .repo
            .clone()
            .unwrap_or_else(|| format!("https://github.com/{}/{}", author, repo));

        println!("Cloning {}", &repo_url);
        git2::build::RepoBuilder::new()
            .clone(&repo_url, &repo_path)
            .context("failed to clone repository")?;
        println!("Cloned {}", &repo_url);

        Ok(())
    }
}
