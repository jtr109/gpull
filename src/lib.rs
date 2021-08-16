use anyhow::Result;
use std::path::PathBuf;

mod docker;
mod google_containers;

pub fn package_images(images: Vec<&str>, path: PathBuf) -> Result<()> {
    for image in images.iter() {
        let ali_image = google_containers::map_image_name(image)?;
        docker::pull(&ali_image)?;
        docker::tag(&ali_image, image)?;
        docker::rmi(&ali_image)?;
    }
    docker::save(path, &images)?;
    Ok(())
}
