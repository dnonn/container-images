/// Zomboid has implemented their start-server.sh in a shitty way
/// where it calls zomboid in a subshell, meaning our signals will not
/// reach the game itself... This works around it
/// Handles communication between threads
use crate::zomboid::handlers::{from_stdin, killer, reader};
use crate::zomboid::commands::{get_start_command};
use std::process::Stdio;
use std::time::Duration;
use tokio::sync::mpsc::{self, Receiver};
use tokio::task::spawn_blocking;
use wait_timeout::ChildExt;

/// Starts and runs the game. Returns a standard lib child and not tokio
/// since tokio is incompatible with "with-timeout" crate, but
/// std stdin can be converted to tokio stdin when we need it.
pub async fn run(install_path: &str, server_parameters: String) -> anyhow::Result<std::process::Child> {
    // Retrieve the command to start the game using the provided installation path
    let command = get_start_command(install_path);
    info!("Running command: {}", command);

    #[allow(unused_mut)] // Else we get compiler warnings on windows due to the following
    let mut gamebuilder = std::process::Command::new(command);
    #[cfg(target_family = "unix")]
    {
        use std::os::unix::prelude::CommandExt;
        // Ensures CTRL+c in the terminal won't be sent directly to the server on UNIX
        gamebuilder.process_group(0);
    }
    // Configure the game process builder
    gamebuilder
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    gamebuilder.args(server_parameters.split(','));

    // Setup a channel for communication between threads
    let (rx, mut tx) = mpsc::channel(1);
    // Spawn a blocking task to start the game asynchronously
    spawn_blocking(move || {
        // This is blocking
        let game = gamebuilder.spawn();

        // Send the spawned game process through the channel
        rx.blocking_send(game)
            .expect("Unable to send child after starting it");
    });

    // Receive the started game process from the channel
    let game = tx.recv().await.expect("Unable to start child")?;
    Ok(game)
}

/// Ensures the game exits properly. Returns status code
pub async fn wait_for(
    mut game: std::process::Child,
    condition: Receiver<()>,
    exit_timeout: Duration,
) -> anyhow::Result<i32> {
    // Take the standard library stdin from the game process
    let stdlib_stdin = game.stdin.take().unwrap();
    // Convert the standard library stdin to tokio's ChildStdin
    let stdin = tokio::process::ChildStdin::from_std(stdlib_stdin)?;

    // Setup channels for communication between threads
    let (stdin_tx, stdin_rx) = mpsc::channel(32);
    let (message_writer, mut message_reader) = mpsc::channel(2);

    // Spawn tasks to handle stdin communication and game killing logic
    tokio::spawn(killer(condition, stdin_tx.clone(), message_writer));
    tokio::spawn(from_stdin(stdin_tx));
    tokio::spawn(reader(stdin_rx, stdin));

    // Wait until the game is attempting to exit
    message_reader.recv().await;

    // Setup channels for status communication
    let (status_writer, mut status_reader) = mpsc::channel(1);

    // Spawn a blocking task to wait for the game to exit
    spawn_blocking(move || {
        let exited = game
            .wait_timeout(exit_timeout)
            .expect("Unable to wait for child");
        let code = match exited {
            Some(status) => {
                info!("Server has exited");
                status.code().unwrap_or(0) // Retrieve the exit code if available
            }
            None => {
                info!("Child timed out, killing it");
                game.kill().expect("Unable to kill child");
                // Wait for the game to exit after killing to avoid zombies
                game.wait()
                    .expect("Unable to wait for child")
                    .code()
                    .unwrap_or(-1) // Indicate improper exit if unable to get exit code
            }
        };
        // Send the exit code through the status channel
        status_writer
            .blocking_send(code)
            .expect("Unable to inform main of exit status");
    });

    // Receive the exit code from the status channel
    let exitcode = status_reader.recv().await.unwrap_or_else(|| {
        error!("status_writer exited without writing to channel!");
        -1 // Indicate an error if unable to retrieve exit code
    });
    Ok(exitcode)
}
