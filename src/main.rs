use anyhow::Result;
use clap::{load_yaml, App};
use gpull::package_images;

fn main() -> Result<()> {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();
    let path = matches.value_of("path").expect("missing argument: path");
    let images: Vec<_> = matches
        .values_of("images")
        .expect("missing argument: image")
        .collect();
    package_images(images, std::path::PathBuf::from(path))?;
    Ok(())
}
