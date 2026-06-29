use bollard::Docker;
use bollard::container::{Config, CreateContainerOptions, StartContainerOptions};
use bollard::image::CreateImageOptions;
use futures_util::stream::TryStreamExt; // Required to handle the pulling stream

#[tokio::main]
async fn run_code(
    container: String,
    filePath: String,
    command: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = Config {
        image: Some(container),
        cmd: Some(vec![command.to_string(), filePath.to_string()]),
        host_config: Some(HostConfig {
            binds: Some(vec!["/tmp/user123:/app".to_string()]),
            auto_remove: Some(true),
            network_mode: Some("none".to_string()),
            memory: Some(256 * 1024 * 1024),
            ..Default::default()
        }),
        ..Default::default()
    };
    Ok(())
}

pub async fn init_run(
    filePath: String,
    language: String,
) -> Result<(), Box<dyn std::error::Error>> {
    if language == "python" {
        run_code("python:3.13-slim", filePath, "python")
    }
    Ok(())
}
