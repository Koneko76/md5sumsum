extern crate walkdir;
use std::env;
use std::process::Command;

fn main() {

    // Declaration of variables //
    
    let mut hashcat = String::from("");
    let mut paths: Vec<String> = vec![];
    let mut cmd = Command::new("sh");
    let mut output;

    // ---------------------- //
    
    for argument in env::args().skip(1) {
        for entry in walkdir::WalkDir::new(argument)
            .into_iter().filter_map(Result::ok).filter(|e| !e.file_type().is_dir())
        {
            paths.push(String::from(entry.path().to_string_lossy()))
        }
    }

    for (x, _y) in paths.iter().enumerate() {

            cmd.arg("-c").arg(format!("md5sum \"{}\"", x));
            output = String::from_utf8_lossy(&cmd.output().unwrap().stdout).to_string();
            let vec: Vec<&str> = output.split(' ').collect();
            hashcat += vec[0];

        }

    cmd.arg("-c").arg(format!("echo \"{}\" | md5sum", hashcat));
    output = String::from_utf8_lossy(&cmd.output().unwrap().stdout).to_string();
    print!("{}", output);
} 


