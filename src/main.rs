use anyhow::{Context, Result};
use clap::Parser;
use log::{debug, error, info, warn};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

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
    #[clap(short, long, required = true)]
    source_dir: PathBuf,
}

#[derive(Debug, Deserialize)]
struct ArtifactProject {
    project: String,
    #[serde(rename = "ref")]
    ref_: String,
    job: String,
    install: HashMap<String, String>,
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();

    info!("Starting gitlab-art-copier");
    debug!("Command line arguments: {:?}", args);

    let yaml_content =
        fs::read_to_string(&args.yaml_file).context(format!("Failed to read YAML file"))?;
    let dst = args.yaml_file.parent().unwrap();

    let projects: Vec<ArtifactProject> =
        serde_yaml::from_str(&yaml_content).context("Failed to parse artifacts.yml")?;

    info!("Using source directory: {:?}", args.source_dir);

    for project in projects {
        if let Some(ref project_filter) = args.project {
            if !project.project.contains(project_filter) {
                debug!(
                    "Skipping project {} (doesn't match filter '{}')",
                    project.project, project_filter
                );
                continue;
            }
        }

        info!("Processing project: {}", project.project);
        if let Err(err) = copy_files(&args.source_dir, dst, &project.install) {
            error!("Error processing project {}: {}", project.project, err);
        }
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
            if destination.exists() {
                info!("Removing existing destination directory: {:?}", destination);
                fs::remove_dir_all(&destination).context(format!(
                    "Failed to remove existing directory: {:?}",
                    destination
                ))?;
            }

            info!(
                "Copying directory recursively from {:?} to {:?}",
                source, destination
            );
            copy_dir_all(&source, &destination).context(format!(
                "Failed to copy directory from {:?} to {:?}",
                source, destination
            ))?;
        } else {
            info!("Copying file from {:?} to {:?}", source, destination);
            fs::copy(&source, &destination).context(format!(
                "Failed to copy file from {:?} to {:?}",
                source, destination
            ))?;
        }

        info!("Successfully copied {:?} to {:?}", source, destination);
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
            if dst_path.exists() && dst_path.is_dir() {
                info!("Removing existing subdirectory: {:?}", dst_path);
                fs::remove_dir_all(&dst_path)?;
            }
            info!("Copying directory from {:?} to {:?}", src_path, dst_path);
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            info!("Copying file from {:?} to {:?}", src_path, dst_path);
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}
