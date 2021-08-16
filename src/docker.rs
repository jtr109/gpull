use anyhow::Result;

use std::path::PathBuf;
use std::process::Command;

pub fn pull(name: &str) -> Result<()> {
    log::debug!("pulling image: {}", name);
    Command::new("docker").arg("pull").arg(name).output()?;
    Ok(())
}

pub fn tag(image_name: &str, new_name: &str) -> Result<()> {
    log::debug!("tagging image {} into {}", image_name, new_name);
    Command::new("docker")
        .arg("tag")
        .arg(image_name)
        .arg(new_name)
        .output()?;
    Ok(())
}

pub fn save(path: PathBuf, images: &Vec<&str>) -> Result<()> {
    log::debug!(
        "saving images {} into {}",
        images.join(" "),
        path.display().to_string(),
    );
    Command::new("docker")
        .arg("save")
        .arg("-o")
        .arg(path)
        .args(images)
        .output()?;
    Ok(())
}

pub fn rmi(image: &str) -> Result<()> {
    log::debug!("removing image: {}", image);
    Command::new("docker").arg("rmi").arg(image).output()?;
    Ok(())
}
