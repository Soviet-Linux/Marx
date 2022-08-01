use std::process::{Command, exit};

pub fn call_cccp(pkg: &Vec<String>) -> Result<String, reqwest::Error> {
    match Command::new("sudo").args(&["cccp", "--aur"]).args(pkg).status() {
        Ok(_) => exit(0),
        Err(_) => exit(1)
    }

}

pub fn call_cccp_single(pkg: &str) -> Result<String, reqwest::Error> {
    match Command::new("sudo").args(&["cccp", "--aur", &pkg]).status() {
        Ok(_) => exit(0),
        Err(_) => exit(1)
    }

}

