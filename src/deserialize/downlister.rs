use reqwest;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::path::Path;
use regex::Regex;
use std::time::SystemTime;
use std::process::Command;

extern crate simplelog;

use simplelog::*;

fn timeconvert(time: &String) -> u64 {
    let re = Regex::new(r"(?P<value>\d+)(?P<unit>[dmsh]*)").unwrap();

    let cap = match re.captures(time) {
        None => panic!("Unknown Timeformat: {}", time),
        Some(x) => x
    };

    let cap_val = match cap.name("value") {
        None => "1",
        Some(x) => x.as_str()
    };

    let cap_unit = match cap.name("unit") {
        None => "s",
        Some(x) => x.as_str()
    };

    let value = cap_val.to_string().parse::<u64>().unwrap();
    if &cap_unit == &"m" {
        return value * 60;
    } else if &cap_unit == &"s" {
        return value;
    } else if &cap_unit == &"h" {
        return value * 60 * 60;
    } else if &cap_unit == &"d" {
        return value * 60 * 60 * 24;
    } else {
        return value;
    }
}

fn is_renewable(filepath: &Path, timeout: &String) -> bool {
    let t = timeconvert(timeout);
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH); 
    let metadata = fs::metadata(filepath).unwrap();
    let mut delta = 0;

    if let Ok(time) = metadata.created() {
        let delta_duration = now.unwrap() - time.duration_since(SystemTime::UNIX_EPOCH).unwrap();
        delta = delta_duration.as_secs();
    }

    if delta > t {
        debug!("Time for renewal. delta is {} seconds", delta);
        return true;
    } else {
        debug!("Renewal in {} seconds", t - delta);
        return false;
    }

}

fn do_action(ip: &str, blockaction: &String, blockaction_v6: &String, timeout: &String) {
    let action: String = blockaction.replace("${IP}",ip);
    debug!("Blockaction6: {}", blockaction_v6);
    debug!("{}", action.replace("${TIMEOUT}",timeout));
    let output = Command::new("sh")
            .arg("-c")
            .arg(action.replace("${TIMEOUT}",timeout))
            .output()
            .expect("failed to execute process");
    debug!("{:?}", String::from_utf8(output.stdout));
}

fn get_from_url(url: String, filepath: &Path, blockaction: &String, blockaction_v6: &String, timeout: &String) {
    let result = reqwest::blocking::get(url).unwrap();
    let body = match result.text() {
        Ok(x) => x,
        Err(e) => { panic!("Download failed {:?}", e) }
    };
    let prefilter = Regex::new(r"^\s*([0-9\.:/]+)").unwrap();

    for line in body.split("\n") {
        match prefilter.captures(line) {
            Some(x) => do_action(x.get(0).unwrap().as_str(), &blockaction, &blockaction_v6, &timeout),
            None => debug!("ignorered: {}", line)
        }
    }

    let mut output = File::create(filepath).expect("whoopsi");
    write!(output, "{}", &body).unwrap_or_else(|error| { 
        warn!("Unable to write file {filepath:?}: {error:?}");
    });
}

pub fn download(name: String, url: String, cachedir: &String, timeout: &String, blockaction: &String, blockaction_v6: &String) {
    info!("processing {}", name);
    let filepath = Path::new(cachedir).join(name);
    if filepath.exists() {
        if is_renewable(&filepath, timeout) {
            info!("Delete file first");
            fs::remove_file(&filepath).unwrap_or_else(|error| {
                warn!("Unable to delete file {filepath:?}: {error:?}");
            });
            get_from_url(url, &filepath, &blockaction, &blockaction_v6, &timeout);
        } else {
            info!("Nothing to renew. For now..");
        }
    }
    else {
        info!("so new");
        get_from_url(url, &filepath, &blockaction, &blockaction_v6, &timeout);
    }
}
