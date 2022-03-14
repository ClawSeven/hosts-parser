use std::net::IpAddr;
use std::str::FromStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str;
use std::io::Read;
extern crate regex;
#[macro_use]
extern crate lazy_static;
use regex::Regex;

lazy_static! {
    static ref HOSTNAME_RE: Regex = Regex::new(r"^[A-Za-z][A-Za-z0-9\.-]*[A-Za-z0-9]$").unwrap();
}

#[derive(Debug, Default, Clone)]
pub struct HostEntry {
    ip: String,
    hostname: Vec<String>
}

#[derive(Debug, Default, Clone)]
pub struct HostFile {
    pub entries: Vec<HostEntry>,
}

impl FromStr for HostEntry {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line = s;
        let re = Regex::new(r"^[A-Za-z][A-Za-z0-9\.-]*[A-Za-z0-9]$").unwrap(); // lazy static
        let slice: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();

        let ip = slice.first().ok_or("malformated ip")?.clone();

        let _ip_addr: IpAddr = match ip.parse() {
            Ok(ip)=> ip,
            Err(_) => return Err("malformated ip")
        };

        let hostname: Vec<String> = (&slice[1..]).iter().take_while(|s| !HOSTNAME_RE.is_match(s))
        .map(|h| h.to_string())
        .collect();
        if hostname.is_empty() {
            return Err("malformated hostname")
        }
        Ok(HostEntry { ip, hostname })
    }
}

impl HostFile {
    pub fn write_to_string(&self) -> String {
        let mut host_string  = String::new();
        let entry_iter = self.entries.iter();

        for entry in entry_iter {
            let mut entry_string = String::new();
            entry_string.push_str(&entry.ip);

            for hostname in entry.hostname.iter() {
                entry_string.push_str(" ");
                let name_string = hostname;
                entry_string.push_str(name_string);
            }
            entry_string.push_str("\n");
            host_string.push_str(&entry_string);
        }

        host_string
    }
}

pub fn parse_hosts_buffer(bytes: &[u8]) -> Result<HostFile, &'static str> {
    // let mut entries = Vec::new();
    let mut hostfile: HostFile = Default::default();
    for (_, line) in bytes.split(|&x| x == b'\n').enumerate() {
        let line = str::from_utf8(line).unwrap();
        let line = line.trim_start();
        match line.chars().next() {
            // comment
            Some('#') => continue,
            // empty line
            None => continue,
            // valid line
            Some(_) => {},
        }
        hostfile.entries.push(line.parse()?);
        // entries.push(line.parse()?);
    }
    Ok(hostfile)
}




pub fn parse_file(path: &Path) -> Result<Vec<HostEntry>, &'static str> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return Err("failed to open file"),
    };
    let mut entries = Vec::new();

    let lines = BufReader::new(file).lines();
    for line in lines {
        if let Err(_) = line {
            return Err("Error reading file");
        }

        let line = line.unwrap();
        let line = line.trim_start();
        match line.chars().next() {
            // comment
            Some('#') => continue,
            // empty line
            None => continue,
            // valid line
            Some(_) => {},
        };

        entries.push(line.parse()?);
    }

    Ok(entries)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main(){
        let hosts = parse_file(&Path::new("/etc/hosts")).unwrap();
        println!("{:?}", hosts);
    }

    #[test]
    fn test_parse_hosts_buffer(){
        let mut buf = Vec::with_capacity(4096);
        let mut f = File::open("/etc/hosts").unwrap();
        f.read_to_end(&mut buf).unwrap();

        let hosts = parse_hosts_buffer(&buf).unwrap();
        println!("{:?}", hosts);
    }

    #[test]
    fn test_write_to_string(){
        let mut buf = Vec::with_capacity(4096);
        let mut f = File::open("/etc/hosts").unwrap();
        f.read_to_end(&mut buf).unwrap();

        let hosts = parse_hosts_buffer(&buf).unwrap();
        println!("{:}", hosts.write_to_string());
    }
}
    