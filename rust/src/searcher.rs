use super::parser::Link;

pub fn search_linkz(linkz : &Vec<Link>, keywords : &Vec<&str>, tags : &Vec<&str>) -> Vec<Link> {
    linkz.iter()
        .filter(|link| link_has_all_keywords(*link, keywords))
        .filter(|link| link_has_all_tags(*link, tags))
        .cloned()
        .collect()
}

fn link_has_all_keywords(linkz : &Link,  keywords : &Vec<&str>) -> bool {
 keywords.iter().all(|i|linkz.url.contains(*i) || linkz.description.contains(*i))
}

fn link_has_all_tags(linkz : &Link,  tags : &Vec<&str>) -> bool {
    tags.iter().all(|i| linkz.tags.iter().any(|t| (*t).contains(*i)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn search_one_matching_tag() {
        let linkz = link_create();
        let res = search_linkz(&linkz, &vec![], &vec!["tag1"]);
        assert_eq!(res.len(),1);
        assert_eq!(res[0].url, linkz[0].url);
    }

    #[test]
    fn search_one_multi_matching_tag() {
        let linkz = link_create();
        let res = search_linkz(&linkz, &vec![], &vec!["tagA"]);
        assert_eq!(res.len(),2);
        assert_eq!(res[0].url, linkz[0].url);
        assert_eq!(res[1].url, linkz[1].url);
    }

    #[test]
    fn search_one_non_matching_tag() {
        let linkz = link_create();
        let res = search_linkz(&linkz, &vec![], &vec!["tagX"]);
        assert_eq!(res.len(),0);
    }

    fn link_create() -> Vec<Link> {
        vec![
            Link {
                url: "http://123/".to_string(),
                description: "Description ABC".to_string(),
                tags: vec!["tag1".to_string(), "tag2".to_string(), "tagA".to_string()]
            },
            Link {
                url: "http://456/".to_string(),
                description: "Description DEF".to_string(),
                tags: vec!["tag3".to_string(), "tag4".to_string(), "tagA".to_string()]
            }]
    }
}
