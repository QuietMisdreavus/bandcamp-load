extern crate hyper;
extern crate json;

use std::io::{BufRead, BufReader};

use hyper::status::StatusCode;

fn main() {
    let link = if std::env::args().len() > 1 {
        std::env::args().nth(1).unwrap()
    }
    else {
        println!("please supply a link as an argument");
        return;
    };

    let client = hyper::Client::new();

    let resp = client.get(&link)
                     .send().unwrap();

    if resp.status == StatusCode::Ok {
        let mut current = String::new();
        let mut trackinfo = String::new();
        let mut artist = String::new();
        let mut in_data = false;

        for line in BufReader::new(resp).lines() {
            let line = line.unwrap();
            if !in_data {
                if line.contains("var TralbumData =") {
                    in_data = true;
                }
            } else {
                if line == "};" {
                    break;
                }

                let real_line = if let Some(pos) = line.find(" //") {
                    &line[..pos+1]
                } else {
                    &line[..]
                }.trim();

                if !real_line.is_empty() {
                    if real_line.starts_with("current") {
                        let start = real_line.find('{').unwrap();
                        let end = real_line.rfind('}').unwrap();
                        current = real_line[start..end+1].to_string();
                    }
                    else if real_line.starts_with("trackinfo") {
                        let start = real_line.find('[').unwrap();
                        let end = real_line.rfind(']').unwrap();
                        trackinfo = real_line[start..end+1].to_string();
                    }
                    else if real_line.starts_with("artist") {
                        let start = real_line.find('"').unwrap();
                        let end = real_line.rfind('"').unwrap();
                        artist = real_line[start+1..end].to_string();
                    }
                }
            }
        }

        if !in_data {
            println!("never found data?");
            return;
        }

        let current = json::parse(&current).unwrap();
        let trackinfo = json::parse(&trackinfo).unwrap();

        println!("{}, by {}", current["title"], artist);
        println!("Released on {}", current["publish_date"]);
        println!("{}", current["about"]);

        println!("");

        let duration: f64 = trackinfo.members().map(|v| v["duration"].as_f64().unwrap()).sum();
        let hours = duration as u32 / (60 * 60);
        if hours > 0 {
            println!("{} tracks, length {}:{:02}:{:02}", trackinfo.len(), hours, ((duration / 60.0) as u32) % 60, (duration as u32 % 60));
        } else {
            println!("{} tracks, length {}:{:02}", trackinfo.len(), (duration / 60.0) as u32, (duration as u32 % 60));
        }

        println!("");

        for (idx, track) in trackinfo.members().enumerate() {
            println!("{}. {}", idx+1, track["title"]);
        }
    } else {
        println!("Bad response from server: {}", resp.status);
    }
}
