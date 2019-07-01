use serde::Deserialize;
use std::fs;
use toml;

#[derive(Deserialize)]
struct XEnv {
    packages: Packages,
}

#[derive(Deserialize)]
struct Packages {
    mesos: String, // TODO: make this pair generic.
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let content = fs::read(".xenv.toml")?;
    let t: XEnv = toml::de::from_slice(&content)?;
    let export_path = format!("/Users/kjeschkies/.mesos/mesos_versions/{}/bin:/Users/kjeschkies/.mesos/mesos_versions/{}/sbin",
                              t.packages.mesos, t.packages.mesos);
    println!("{}", export_path);
    Ok(())
}
