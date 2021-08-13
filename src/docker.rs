use anyhow::Result;

use std::path::PathBuf;
use std::process::Command;

pub fn pull(name: &str) -> Result<()> {
    Command::new("docker").arg("pull").arg(name).output()?;
    Ok(())
}

pub fn tag(image_name: &str, new_name: &str) -> Result<()> {
    Command::new("docker")
        .arg("tag")
        .arg(image_name)
        .arg(new_name)
        .output()?;
    Ok(())
}

pub fn save(path: PathBuf, images: &Vec<String>) -> Result<()> {
    Command::new("docker")
        .arg("save")
        .arg("-o")
        .arg(path)
        .args(images)
        .output()?;
    Ok(())
}

pub fn rmi(image: &str) -> Result<()> {
    Command::new("docker").arg("rmi").arg(image).output()?;
    Ok(())
}
