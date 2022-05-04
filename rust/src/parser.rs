use serde_json::Value;

#[derive(Debug,Clone)]
pub struct Link {
    pub url: String,
    pub description: String,
    pub tags: Vec<String>,
}

pub fn parse_contents(contents: &Vec<Value>) -> Vec<Link> {
    let mut result = Vec::new();
    for content in contents {
        if let Some(type_) = content["type"].as_str() {
            match type_ {
                "headline" | "section" => {
                    if let Some(properties) = content["properties"].as_object() {
                        let mut tags :Vec<String> = Vec::new();
                        if let Some(tags_v) = properties.get("tags") {
                            if tags_v.is_array() {
                                let tags_array = tags_v.as_array().unwrap();
                                for t in tags_array {
                                    if t.is_string() {
                                        tags.push(t.as_str().unwrap().to_string());
                                    }
                                }
                            }
                        }
                        if let Some(title) = properties.get("title") {
                            if let Some(title_array) = title.as_array() {
                                for title_entry in title_array {
                                    if title_entry.is_object() {
                                        let title_map = title_entry.as_object().unwrap();
                                        let mut link: Option<&str> = None;
                                        let mut description = None;
                                        if (title_map.contains_key("properties") )
                                            && (title_map["properties"]["raw-link"].is_string()) {
                                            let p = title_map["properties"].as_object().unwrap();
                                            link = p.get("raw-link").unwrap().as_str();
                                        }
                                        if title_map.contains_key("contents") && title_map["contents"].is_array() {
                                            let c = title_map["contents"].as_array().unwrap();
                                            let mut desc = String::new();
                                            for s in c.iter() {
                                                desc.push_str(s.as_str().unwrap());
                                                desc.push(' ');
                                            }
                                            description = Some(desc);

                                        }
                                        if link.is_some() && description.is_some() {
                                            result.push(Link {
                                                url: link.unwrap().to_string(),
                                                description: description.unwrap().to_string(),
                                                tags: tags.clone(),
                                            })
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {
                }
            }
        }
        if let Some(child_contents) = content["contents"].as_array() {
            result.extend(parse_contents(child_contents));
        }
    }
    result
}
