use bollard::Docker;
use bollard::container::{Config, CreateContainerOptions, LogOutput, StartContainerOptions};
use bollard::exec::{CreateExecOptions, StartExecResults};
use bollard::models::HostConfig;
use futures_util::StreamExt;

async fn ensure_container_running(
    docker: &Docker,
    container_name: &str,
    image: &str,
    host_storage_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let containers = docker
        .list_containers(Some(bollard::container::ListContainersOptions::<String> {
            all: true,
            ..Default::default()
        }))
        .await?;

    let exists = containers.iter().any(|c| {
        c.names.as_ref().map_or(false, |names| {
            names.iter().any(|n| n == &format!("/{}", container_name))
        })
    });

    if !exists {
        let host_config = HostConfig {
            binds: Some(vec![format!("{}:/app/store:ro", host_storage_dir)]),
            ..Default::default()
        };

        let config = Config {
            image: Some(image.to_string()),
            cmd: Some(vec!["sleep".to_string(), "infinity".to_string()]),
            host_config: Some(host_config),
            ..Default::default()
        };

        docker
            .create_container(
                Some(CreateContainerOptions {
                    name: container_name,
                    ..Default::default()
                }),
                config,
            )
            .await?;

        docker
            .start_container(container_name, None::<StartContainerOptions<String>>)
            .await?;
        println!("Started container: {}", container_name);
    } else {
        let container = containers.iter().find(|c| {
            c.names.as_ref().map_or(false, |names| {
                names.iter().any(|n| n == &format!("/{}", container_name))
            })
        });
        if let Some(c) = container {
            if c.state != Some("running".to_string()) {
                docker
                    .start_container(container_name, None::<StartContainerOptions<String>>)
                    .await?;
            }
        }
    }

    Ok(())
}

async fn run_in_container(
    docker: &Docker,
    container_name: &str,
    container_file_path: &str,
    command: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    println!("Container: {}", container_name);
    println!("Command: {}", command);
    println!("File path (in container): {}", container_file_path);

    let exec = docker
        .create_exec(
            container_name,
            CreateExecOptions {
                attach_stdout: Some(true),
                attach_stderr: Some(true),
                cmd: Some(vec![command.to_string(), container_file_path.to_string()]),
                ..Default::default()
            },
        )
        .await?;

    let mut output_text = String::new();
    match docker.start_exec(&exec.id, None).await? {
        StartExecResults::Attached { mut output, .. } => {
            while let Some(msg) = output.next().await {
                match msg? {
                    LogOutput::StdOut { message } => {
                        let s = String::from_utf8_lossy(&message).to_string();
                        println!("{}", s);
                        output_text.push_str(&s);
                    }
                    LogOutput::StdErr { message } => {
                        let s = String::from_utf8_lossy(&message).to_string();
                        eprintln!("{}", s);
                        output_text.push_str(&s);
                    }
                    _ => {}
                }
            }
        }
        StartExecResults::Detached => {
            println!("Exec started in detached mode");
        }
    }

    Ok(output_text)
}

pub async fn init_run(
    file_path: String,
    language: String,
) -> Result<String, Box<dyn std::error::Error>> {
    println!("init_run()");

    let docker = Docker::connect_with_local_defaults()?;
    let container_name = "python-worker";
    let image = "my-python-runner:latest";
    let host_storage_dir = std::env::var("STORAGE_DIR").unwrap_or_else(|_| "./store".to_string());
    let host_storage_dir = std::fs::canonicalize(&host_storage_dir)?
        .to_string_lossy()
        .to_string();

    ensure_container_running(&docker, container_name, image, &host_storage_dir).await?;

    let container_file_path = file_path.replace(&host_storage_dir, "/app/store");

    if language == "python" {
        println!("running code");
        let output = run_in_container(&docker, container_name, &container_file_path, "python").await?;
        return Ok(output);
    }

    Ok("Language not supported".to_string())
}