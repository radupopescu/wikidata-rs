use serde_json::from_str;
use serde_json::value::Value as SDValue;

use std::collections::HashMap;

use errors::*;

pub type WikiTitles = HashMap<String, String>;

pub fn parse_item(line: &str, languages: &Vec<String>) -> Result<Option<WikiTitles>,
                                                                 WikiError> {
    let item: SDValue = from_str(line)?;

    let mut sites = WikiTitles::new();
    if let Some(sitelinks) = item.find("sitelinks") {
        for l in languages {
            let link = format!("{}wiki", l);
            match sitelinks.find(&link) {
                Some(res) => {
                    if let Some(title) = res.find("title") {
                        sites.insert(l.to_owned(), title.to_string());
                    }
                },
                None => (),
            }
        }
    }
    let result = if sites.len() == languages.len() {
        Some(sites)
    } else {
        None
    };
    Ok(result)
}

