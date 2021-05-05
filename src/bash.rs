use std::process::Command;

pub fn exec(cmd: String) -> String {
    let stdout = Command::new("/bin/bash")
        .arg("-c")
        .arg(cmd)
        
    
        .output()
        .unwrap()
        .stdout;

    let s = std::str::from_utf8(&stdout).unwrap();
    s.to_string()
}


