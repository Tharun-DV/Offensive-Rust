use std::env;
use std::process::exit;
use std::process::Command;

fn execute_cmd(cmd: &str) -> String {
    // cmd.exe /c commands
    let setup: String = "/c".to_owned();
    let fullcmd: String = setup + cmd;

    let cmds: Vec<&str> = fullcmd.split(" ").collect();
    let res = Command::new("cmd.exe").args(&cmds).output().unwrap();
    let stdout = String::from_utf8_lossy(res.stdout.as_slice());
    let stderr = String::from_utf8_lossy(res.stderr.as_slice());

    if stdout.len() > 2 {
        return String::from(stdout);
    } else {
        return String::from(stderr);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("[+] Usage : {} Command", args[0]);
        println!("[+] Usage : {} ls", args[0]);
        exit(0);
    }
    let rs = execute_cmd(&args[1]);
    println!("{}",rs);
}