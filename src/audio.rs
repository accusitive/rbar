    use std::process::Command;

    pub fn get_sink_vol(sink: u8) -> i32 {
        let stdout = Command::new("/usr/bin/pamixer")
            .arg("--get-volume")
            .arg("--sink")
            .arg(format!("{}", sink)
        )
            .output()
            .unwrap()
            .stdout;
    
        let s = std::str::from_utf8(&stdout).unwrap();
        s.trim().parse::<i32>().unwrap_or(-1)
    }
    

