use super::parser::Link;

pub fn search_linkz(linkz : &Vec<Link>,
                    keywords : &Vec<&str>,
                    tags : &Vec<&str>,
                    keywords_case : bool,
                    tags_case : bool) ->
                                                                                                           Vec<Link> {
    linkz.iter()
        .filter(|link| link_has_all_keywords(*link, keywords, keywords_case))
        .filter(|link| link_has_all_tags(*link, tags, tags_case))
        .cloned()
        .collect()
}

fn link_has_all_keywords(linkz : &Link,  keywords : &Vec<&str>, keywords_case: bool) -> bool {
    match keywords_case {
        true => keywords.iter().all(|i| { linkz.url.contains(*i) || linkz.description.contains(*i) }),
        false => keywords.iter().all(|i| { linkz.url.to_lowercase().contains((*i).to_lowercase().as_str())
            || linkz.description.to_lowercase().contains((*i).to_lowercase().as_str()) })
    }
}

fn link_has_all_tags(linkz : &Link,  tags : &Vec<&str>, tags_case: bool) -> bool {
    match tags_case {
        true => tags.iter().all(|i| linkz.tags.iter().any(|t| (*t).contains(*i))),
        false => tags.iter().all(|i| linkz.tags.iter().any(|t|
            (*t).to_lowercase().contains((*i).to_lowercase().as_str())))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn search_case_sensitive_tags() {
        let linkz = link_create();
        let res = search_linkz(&linkz, &vec![], &vec!["tagA"], false, true);
        assert_eq!(res.len(),2);
        let res = search_linkz(&linkz, &vec![], &vec!["taga"], false, false);
        assert_eq!(res.len(),2);
        let res = search_linkz(&linkz, &vec![], &vec!["Taga"], false, true);
        assert_eq!(res.len(),0);
    }

    #[test]
    fn search_case_sensitive_keywords() {
        let linkz = link_create();
        let res = search_linkz(&linkz, &vec!["Descr"], &vec![], true, false);
        assert_eq!(res.len(),2);
        let res = search_linkz(&linkz, &vec!["descr"], &vec![], false, false);
        assert_eq!(res.len(),2);
        let res = search_linkz(&linkz, &vec!["descr"], &vec![], true, false);
        assert_eq!(res.len(),0);
    }

    #[test]
    fn search_one_matching_tag() {
        let linkz = link_create();
        let res = search_linkz(&linkz, &vec![], &vec!["tag1"], false, false);
        assert_eq!(res.len(),1);
        assert_eq!(res[0].url, linkz[0].url);
        let res = search_linkz(&linkz, &vec![], &vec!["tag3"], false, false);
        assert_eq!(res.len(),1);
        assert_eq!(res[0].url, linkz[1].url);
        let res = search_linkz(&linkz, &vec![], &vec!["tagA"], false, false);
        assert_eq!(res.len(),2);
        assert_eq!(res[0].url, linkz[0].url);
        assert_eq!(res[1].url, linkz[1].url);
    }

    #[test]
    fn search_two_matching_tags() {
        let linkz = link_create();
        let res = search_linkz(&linkz, &vec![], &vec!["tag1","tag2"], false, false);
        assert_eq!(res.len(),1);
        assert_eq!(res[0].url, linkz[0].url);
        let res = search_linkz(&linkz, &vec![], &vec!["tag3","tag4"], false, false);
        assert_eq!(res.len(),1);
        assert_eq!(res[0].url, linkz[1].url);
        let res = search_linkz(&linkz, &vec![], &vec!["tag1","tag4"], false, false);
        assert_eq!(res.len(),0);
    }

    #[test]
    fn search_one_multi_matching_tag() {
        let linkz = link_create();
        let res = search_linkz(&linkz, &vec![], &vec!["tagA"], false, false);
        assert_eq!(res.len(),2);
        assert_eq!(res[0].url, linkz[0].url);
        assert_eq!(res[1].url, linkz[1].url);
    }

    #[test]
    fn search_one_non_matching_tag() {
        let linkz = link_create();
        let res = search_linkz(&linkz, &vec![], &vec!["tagX"], false, false);
        assert_eq!(res.len(),0);
    }

    #[test]
    fn search_one_matching_keyword_and_tag() {
        let linkz = link_create();
        let res = search_linkz(&linkz, &vec!["ABC"], &vec!["tag2"], false, false);
        assert_eq!(res.len(),1);
        assert_eq!(res[0].url, linkz[0].url);
        let res = search_linkz(&linkz, &vec!["DEF"], &vec!["tag2"], false, false);
        assert_eq!(res.len(),0);
        let res = search_linkz(&linkz, &vec!["XYZ"], &vec!["tagA"], false, false);
        assert_eq!(res.len(),2);
        assert_eq!(res[0].url, linkz[0].url);
        assert_eq!(res[1].url, linkz[1].url);
    }

    #[test]
    fn search_two_matching_keyword() {
        let linkz = link_create();
        let res = search_linkz(&linkz, &vec!["ABC","GHI"], &vec![], false, false);
        assert_eq!(res.len(),1);
        assert_eq!(res[0].url, linkz[0].url);
        let res = search_linkz(&linkz, &vec!["DEF","JKL"], &vec![], false, false);
        assert_eq!(res.len(),1);
        assert_eq!(res[0].url, linkz[1].url);
        let res = search_linkz(&linkz, &vec!["ABC","JKL"], &vec![], false, false);
        assert_eq!(res.len(),0);
    }

    #[test]
    fn search_one_multi_matching_keyword() {
        let linkz = link_create();
        let res = search_linkz(&linkz, &vec!["XYZ"], &vec![], false, false);
        assert_eq!(res.len(),2);
        assert_eq!(res[0].url, linkz[0].url);
        assert_eq!(res[1].url, linkz[1].url);
    }

    #[test]
    fn search_one_non_matching_keyword() {
        let linkz = link_create();
        let res = search_linkz(&linkz, &vec!["notthere"], &vec![], false, false);
        assert_eq!(res.len(),0);
    }

    fn link_create() -> Vec<Link> {
        vec![
            Link {
                url: "http://123/".to_string(),
                description: "Description ABC GHIXYZ".to_string(),
                tags: vec!["tag1".to_string(), "tag2".to_string(), "tagA".to_string()]
            },
            Link {
                url: "http://456/".to_string(),
                description: "Description DEF XYZJKL".to_string(),
                tags: vec!["tag3".to_string(), "tag4".to_string(), "tagA".to_string()]
            }]
    }
}
