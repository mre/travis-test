extern crate regex;
extern crate reqwest;
extern crate pencil;

use regex::Regex;
use pencil::{Pencil, Request, Response, PencilResult};

fn check_pull_request(request: &mut Request) -> PencilResult {
    // Warning! Big scary regex ahead.
    let re = Regex::new(r##"^\* \[.*?\]\((.*?)\) \[(PROPRIETARY|OSS|PROPRIETARY/OSS)\] - ([^ ][A-Za-z0-9/'.,-_ ()+#"`!&]*)$"##).unwrap();

    let payload = match request.args().get("payload") {
        Some(payload) => payload as &str,
        None => {
            let mut response = Response::from(format!("Cannot read payload"));
            response.status_code = 404;
            return Ok(response);
        }
    };

    // let entry = match line {
    // Ok(entry) => entry,
    // Err(e) => {
    // let mut response = Response::from(format!("Cannot read input line: {:?}", e));
    // response.status_code = 404;
    // Ok(response)
    // }
    // };
    //
    // if !re.is_match(&entry) {
    // println!("Invalid format for project entry: {:?}", entry);
    // process::exit(1)
    // }
    //
    // let cap = re.captures(&entry).unwrap();
    //
    // let project_url = cap.at(1);
    // let url = match project_url {
    // Some(url) => url,
    // _ => {
    // println!("Could not parse project url from entry: {:?}", entry);
    // process::exit(2)
    // }
    // };
    //
    // let resp_result = reqwest::get(url);
    // let resp = match resp_result {
    // Ok(resp) => resp,
    // Err(e) => {
    // println!("Error reading {:?}: {}", url, e);
    // process::exit(3)
    // }
    // };
    // if !resp.status().is_success() {
    // println!("Project url is not reachable: {:?}", url);
    // process::exit(3)
    // }
    //
    // let project_description = cap.at(3).unwrap();
    //
    // if project_description.len() < 20 {
    // println!("Project description too short: {:?}", project_description);
    // process::exit(3)
    // }
    //
    // if project_description.len() > 90 {
    // println!("Project description too long: {:?}", project_description);
    // process::exit(4)
    // }
    //
    Ok(Response::from("All fine!"))
}
fn main() {
    let mut app = Pencil::new("/");
    app.post("/payload", "check_pull_request", check_pull_request);
    app.run("127.0.0.1:8080");
}
