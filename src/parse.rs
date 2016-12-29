//use time::precise_time_ns;
use serde_json::from_str;
use serde_json::value::Value as SDValue;
use std::collections::HashMap;

use errors::*;

pub type WikiTitles = HashMap<String, String>;

pub fn parse_item(line: &str, languages: &[String])
                  -> Result<Option<WikiTitles>, WikiError> {
    //let t0 = precise_time_ns();
    let end = if line.ends_with(",\n") {
        line.len() - 2
    } else {
        line.len() - 1
    };
    let item: SDValue = from_str(&line[0..end])?;

    let mut sites = WikiTitles::new();
    if let Some(sitelinks) = item.find("sitelinks") {
        for l in languages {
            let link = format!("{}wiki", l);
            if let Some(res) = sitelinks.find(&link) {
                if let Some(title) = res.find("title") {
                    sites.insert(l.to_owned(),
                                 title.to_string());
                }
            }
        }
    }
    let result = if sites.len() == languages.len() {
        Some(sites)
    } else {
        None
    };
    //let t1 = precise_time_ns();
    //println!("parse_item: {} us", (t1 - t0) / 1000 );
    Ok(result)
}

