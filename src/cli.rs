use std::process::Command;

use crate::builder::Cmd;

#[derive(Debug)]
pub enum CliErr {
    RunningErr,
    StdOutErr,
}

pub fn run(cmd: Cmd) -> Result<(), CliErr> {
    let invocation = cmd.invocation.clone();
    let args = cmd.args.clone();
    let mut process_cmd = Command::new(cmd.invocation);
    process_cmd.args(cmd.args);

    let cmd_to_be_executed = process_cmd.get_args().collect::<Vec<&std::ffi::OsStr>>();

    let mut cmd_string = String::new();
    for cmd in &cmd_to_be_executed {
        cmd_string.push_str(cmd.to_str().unwrap());
    }

    println!(
        "The command that will be executed is:\n\n\t{} {}\n\nAre you sure you wanna execute? (Y/n): ",
        invocation, cmd_string
    );

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    if buf != "Y" {
        println!("Exiting...");
        return Ok(());
    }

    // let output = process_cmd.output().or(Err(CliErr::RunningErr))?;

    // let stdout = String::from_utf8(output.stdout).or(Err(CliErr::StdOutErr))?;
    // println!("{stdout}");
    Ok(())
}
