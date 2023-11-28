use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Clone, Serialize, Deserialize)]
pub struct Exit {
    pub pf_id: String,
    pub display_name: String,
}

impl Exit {
    pub fn add_host(&self, ip: &str) {
        let pf_id = &self.pf_id;
        // println!("++ pfctl -t{pf_id} -Tadd {ip}");
        Command::new("pfctl")
            .args([&format!("-t{pf_id}"), "-Tadd", ip])
            .status()
            .expect("command failed to run");
    }

    pub fn delete_host(&self, ip: &str) {
        let pf_id = &self.pf_id;
        // println!("++ pfctl -t{pf_id} -Tdelete {ip}");
        Command::new("pfctl")
            .args([&format!("-t{pf_id}"), "-Tdelete", ip])
            .status()
            .expect("command failed to run");
    }

    pub fn list_hosts(&self) -> Vec<String> {
        let pf_id = &self.pf_id;
        // println!("++ pfctl -t{pf_id} -Tshow");
        // vec![]
        let stdout = Command::new("pfctl")
            .args([&format!("-t{pf_id}"), "-Tshow"])
            .output()
            .expect("command failed to run")
            .stdout;
        let output = String::from_utf8(stdout).unwrap();
        output.lines().map(|s| s.trim().to_owned()).collect()
    }
}
