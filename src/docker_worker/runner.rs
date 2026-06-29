use bollard::Docker;
use bollard::exec::{CreateExecOptions, StartExecResults};
use futures_util::StreamExt;

async fn run_in_existing_container(
    container_name: &str,
    file_path: &str,
    command: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let docker = Docker::connect_with_local_defaults()?;

    println!("Container: {}", container_name);
    println!("Command: {}", command);
    println!("File path: {}", file_path);

    let exec = docker
        .create_exec(
            container_name,
            CreateExecOptions {
                attach_stdout: Some(true),
                attach_stderr: Some(true),
                cmd: Some(vec![command, file_path]),
                ..Default::default()
            },
        )
        .await?;

    match docker.start_exec(&exec.id, None).await? {
        StartExecResults::Attached { mut output, .. } => {
            while let Some(msg) = output.next().await {
                println!("{:?}", msg?);
            }
        }

        StartExecResults::Detached => {
            println!("Exec started in detached mode");
        }
    }

    Ok(())
}
pub async fn init_run(
    file_path: String,
    language: String,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("init_run()");

    if language == "python" {
        println!("running code");
        run_in_existing_container("python-worker", &file_path, "python").await?
    }

    Ok(())
}
