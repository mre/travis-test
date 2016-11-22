extern crate regex;
extern crate reqwest;
use std::process;
use std::io;
use std::io::BufRead;

use regex::Regex;

fn main() {

    // Warning! Big scary regex ahead.
    let re = Regex::new(r##"^\* \[.*?\]\((.*?)\) \[(PROPRIETARY|OSS|PROPRIETARY/OSS)\] - ([^ ][A-Za-z0-9/'.,-_ ()+#"`!&]*)$"##).unwrap();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {

        let entry = match line {
            Ok(entry) => entry,
            Err(e) => {
                println!("Cannot read input line: {:?}", e);
                process::exit(1)
            }
        };

        if !re.is_match(&entry) {
            println!("Invalid format for project entry: {:?}", entry);
            process::exit(1)
        }

        let cap = re.captures(&entry).unwrap();

        let project_url = cap.at(1);
        let url = match project_url {
            Some(url) => url,
            _ => {
                println!("Could not parse project url from entry: {:?}", entry);
                process::exit(2)
            }
        };

        let resp_result = reqwest::get(url);
        let resp = match resp_result {
            Ok(resp) => resp,
            Err(e) => {
                println!("Error reading {:?}: {}", url, e);
                process::exit(3)
            }
        };
        if !resp.status().is_success() {
            println!("Project url is not reachable: {:?}", url);
            process::exit(3)
        }

        let project_description = cap.at(3).unwrap();

        if project_description.len() < 20 {
            println!("Project description too short: {:?}", project_description);
            process::exit(3)
        }

        if project_description.len() > 90 {
            println!("Project description too long: {:?}", project_description);
            process::exit(4)
        }

    }

}
