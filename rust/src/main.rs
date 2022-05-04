use clap::{arg, command, ErrorKind};
use std::fs::File;
use std::{error, io};
use error::Error;
use std::io::{BufRead, BufReader};
use serde_json::{Result as SerdeResult, Value};

mod parser;
mod searcher;

fn main() -> Result<(),Box<dyn Error>>{
    let mut cmd = command!()
        .arg(arg!(-j --json [FILE] "location of the json file to parse. stdin will be used otherwise."))
        .arg(arg!(-b --brief "Output only URLs. If not given, output URLs followed by <TAB> and the description."))
        .arg(arg!(-t --tag [TAG] ... "show only links with these tags"))
        .arg(arg!([KEYWORD] ... "show only links with those keywords in the URL or description."));

    let matches = cmd.get_matches_mut();
    let json_file = matches.value_of("json");
    let brief = matches.is_present("brief");
    let keywords: Vec<&str> = matches.values_of("KEYWORD").unwrap_or_default().collect();
    let tags: Vec<&str> = matches.values_of("tag").unwrap_or_default().collect();

    let reader: Box<dyn BufRead> = match json_file {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) => {
            let input_file = File::open(filename);
            if let Err(e) = input_file {
                cmd.error(
                    ErrorKind::Io,
                    format!("Got '{}' during opening of '{}'", e, json_file.unwrap()),
                ).exit();
            }
            Box::new(BufReader::new(input_file.unwrap()))
        }
    };
    let mut json_str = String::new();
    for line in reader.lines() {
        json_str.push_str(line.unwrap().as_str());
        json_str.push('\n');
    }
    let json_result : SerdeResult<Value> = serde_json::from_str(json_str.as_str());
    if let Err(e) = json_result {
        cmd.error(
            ErrorKind::InvalidValue,
            format!("JSON Parser error: '{}' during parsing of '{}'", e, json_file.unwrap()),
        ).exit();
    }
    let json = json_result.unwrap();
    if let Some(contents) = json["contents"].as_array() {
        let linkz = parser::parse_contents(contents);
        let result = searcher::search_linkz(&linkz,&keywords,&tags);
        for r in result.iter() {
            if brief {
                println!("{}",r.url);
            } else {
                println!("{}\t{}",r.url,r.description)
            }
        }
    }
    Ok(())
}
