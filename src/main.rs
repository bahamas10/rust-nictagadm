use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

struct NicTag {
    name: String,
    mac_address: Option<String>,
    link: Option<String>,
    typ: Option<String>,
}

impl fmt::Display for NicTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<NicTag '{}'>", self.name)
    }
}

impl fmt::Debug for NicTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<NicTag '{}': mac_address {}, link {}, type {}>",
           self.name,
           format_optional_string(&self.mac_address),
           format_optional_string(&self.link),
           format_optional_string(&self.typ))
    }
}

// return a reference to the string or "-" if it is unset
fn format_optional_string<'a>(s: &'a Option<String>) -> &'a str {
    match s {
        Some(s) => s,
        None => "-"
    }
}

// print a warning for an invalid line number
fn invalid_line(fname: &str, line_number: u32, line: &str) {
    eprintln!("error: file {} line {} invalid: '{}'", fname, line_number, line);
}

// parse the /tmp/.nic-tags formatted file created by this program.
// because we create this file if it doesn't exist (stored in tmp so cleared on reboot), we can be
// very strict with the syntax checking
fn parse_tags_file(fname: &str, file: File) -> HashMap<String, NicTag> {
    let mut nic_tags = HashMap::new();
    let mut line_number = 0;

    for line in BufReader::new(file).lines() {
        let line = line.expect("failed to read line");
        line_number += 1;

        let spl: Vec<&str> = line.split('=').collect();

        if spl.len() != 2 {
            invalid_line(fname, line_number, &line);
            eprintln!("try deleting /tmp/.nic-tags and running again");
            exit(1);
        }

        let name = &spl[0];
        let mac_address = &spl[1];

        let nic_tag = NicTag {
            name: name.to_string(),
            mac_address: Some(mac_address.to_string()),
            link: None,
            typ: Some(String::from("normal")),
        };

        nic_tags.insert(name.to_string(), nic_tag);
    }

    nic_tags
}

// parse the vanilla smartos /usbkey/config file
fn parse_usb_file(fname: &str, file: File) -> HashMap<String, NicTag> {
    let mut nic_tags = HashMap::new();
    let mut line_number = 0;

    for line in BufReader::new(file).lines() {
        let line = line.expect("failed to read line");
        line_number += 1;

        if line.trim().is_empty() || line.starts_with("#") {
            continue;
        }

        let spl: Vec<&str> = line.splitn(2, '=').collect();

        if spl.len() != 2 {
            invalid_line(fname, line_number, &line);
            exit(1);
        }

        let name = &spl[0];

        if ! name.ends_with("_nic") {
            continue;
        }

        let mac_address = &spl[1];

        let nic_tag = NicTag {
            name: name.to_string(),
            mac_address: Some(mac_address.to_string()),
            link: None,
            typ: Some(String::from("normal")),
        };

        nic_tags.insert(name.to_string(), nic_tag);
    }

    nic_tags
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];

    let fname = &args[0];
    let file = File::open(fname).expect("Failed to open file");

    let nic_tags: HashMap<String, NicTag> = match fname.as_str() {
        "tags.txt" => parse_tags_file(&fname, file),
        "usb-datadyne.txt" => parse_usb_file(&fname, file),
        "usb-portal.txt" => parse_usb_file(&fname, file),
        _ => panic!("unknown file"),
    };

    println!("NAME\tMACADDRESS\tLINK\tTYPE");
    for (_name, value) in nic_tags {
        println!("{}\t{}\t{}\t{}",
            value.name,
            format_optional_string(&value.mac_address),
            format_optional_string(&value.link),
            format_optional_string(&value.typ));
    }
}
