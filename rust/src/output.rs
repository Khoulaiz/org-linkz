use crate::Link;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize)]
struct AlfredResult {
    items: Vec<Item>
}

#[skip_serializing_none]
#[derive(Serialize)]
struct Item {
    uid: Option<String>,
    title: String,
    subtitle: Option<String>,
    arg: String,
}

pub fn alfred_output(result: Vec<Link>) -> String {
    let items:Vec<Item> = result.iter()
        .map(|l| Item {
            uid: None,
            title: (*l).description.to_string(),
            subtitle: Some((*l).url.to_string()),
            arg: (*l).url.to_string()
        }).collect();
    let alfred_json = AlfredResult {items};
    serde_json::to_string(&alfred_json).unwrap()
}
