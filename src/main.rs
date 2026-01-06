use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow};
use clap::Parser;
use log::trace;
use log::{debug, error, info};
use owo_colors::OwoColorize;
use serde::Deserialize;

mod logger;

#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about = "Copy files based on artifacts.yml configuration"
)]
struct Args {
    /// Path to the artifacts.yml file
    yaml_file: PathBuf,

    /// Only process projects whose name contains this string
    #[clap(short, long)]
    project: Option<String>,

    /// Source directory
    #[clap(short, long, default_value = ".")]
    source_dir: PathBuf,
}

#[derive(Debug, Deserialize)]
struct ArtifactProject {
    project: String,
    install: HashMap<String, String>,
}

fn main() -> Result<()> {
    logger::init();

    let args = Args::parse();

    let yaml_content =
        fs::read_to_string(&args.yaml_file).context("Failed to read YAML file".to_string())?;
    let dst = args.yaml_file.parent().unwrap();

    let projects: Vec<ArtifactProject> =
        serde_yaml::from_str(&yaml_content).context("Failed to parse artifacts.yml")?;

    trace!("Using source directory: {:?}", args.source_dir);

    let mut processed_project = false;
    for project in projects {
        if let Some(ref project_filter) = args.project
            && !project.project.contains(project_filter)
        {
            debug!(
                "Skipping project {} (doesn't match filter '{}')",
                project.project, project_filter
            );
            continue;
        }

        info!("Processing project: {}", project.project);
        if let Err(err) = copy_files(&args.source_dir, dst, &project.install) {
            error!("Error processing project {}: {}", project.project, err);
            continue;
        }
        processed_project = true;
    }

    if !processed_project {
        return Err(anyhow!("Did not process project"));
    }

    Ok(())
}

fn copy_files(source_dir: &Path, dst: &Path, install_map: &HashMap<String, String>) -> Result<()> {
    debug!("Processing {} file mappings", install_map.len());

    for (source_path, dest_path) in install_map {
        let source = if source_path == "." {
            source_dir.to_path_buf()
        } else {
            source_dir.join(source_path)
        };

        let destination = dst.join(PathBuf::from(dest_path));

        debug!("Copying from {:?} to {:?}", source, destination);

        if let Some(parent) = destination.parent() {
            debug!("Ensuring parent directory exists: {:?}", parent);
            fs::create_dir_all(parent)
                .context(format!("Failed to create directory: {:?}", parent))?;
        }

        if source.is_dir() {
            trace!(
                "Copying directory recursively from {:?} to {:?}",
                source, destination
            );
            copy_dir_all(&source, &destination).context(format!(
                "Failed to copy directory from {:?} to {:?}",
                source, destination
            ))?;
        } else {
            info!(
                "Copying file from {:?} to {:?}",
                source.green(),
                destination.green()
            );
            fs::copy(&source, &destination).context(format!(
                "Failed to copy file from {:?} to {:?}",
                source, destination
            ))?;
        }

        trace!("Successfully copied {:?} to {:?}", source, destination);
    }

    Ok(())
}

fn copy_dir_all(src: &Path, dst: &Path) -> io::Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if ty.is_dir() {
            trace!(
                "Copying directory from {:?} to {:?}",
                src_path.blue(),
                dst_path.blue()
            );
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            if fs::exists(&dst_path)? {
                trace!("Removing {:?}", dst_path);
            }
            info!(
                "Copying file from {:?} to {:?}",
                src_path.green(),
                dst_path.green()
            );
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}
