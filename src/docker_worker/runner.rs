use bollard::Docker;
use bollard::container::{Config, CreateContainerOptions, LogsOptions, StartContainerOptions};
use bollard::models::HostConfig;
use futures_util::stream::TryStreamExt;
async fn run_code(
    image: String,
    file_path: String,
    command: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let docker = Docker::connect_with_local_defaults()?;

    let config = Config {
        image: Some(image),
        cmd: Some(vec![command, file_path]),
        host_config: Some(HostConfig {
            binds: Some(vec!["/tmp/user123:/app".to_string()]),
            auto_remove: Some(true),
            network_mode: Some("none".to_string()),
            memory: Some(256 * 1024 * 1024),
            ..Default::default()
        }),
        attach_stdout: Some(true),
        attach_stderr: Some(true),
        ..Default::default()
    };

    // Create container
    let container = docker
        .create_container(None::<CreateContainerOptions<String>>, config)
        .await?;

    // Start container
    docker
        .start_container(&container.id, None::<StartContainerOptions<String>>)
        .await?;

    // Read logs
    let mut logs = docker.logs(
        &container.id,
        Some(LogsOptions::<String> {
            stdout: true,
            stderr: true,
            follow: true,
            ..Default::default()
        }),
    );

    while let Some(log) = logs.try_next().await? {
        print!("{:?}", log);
    }

    Ok(())
}

pub async fn init_run(
    file_path: String,
    language: String,
) -> Result<(), Box<dyn std::error::Error>> {
    if language == "python" {
        println!("running code");
        run_code(
            "python:3.13-slim".to_string(),
            file_path,
            "python".to_string(),
        )
        .await?
    }

    Ok(())
}
