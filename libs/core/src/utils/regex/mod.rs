use regex::Regex;

pub fn regex_get(regex_str: &str, text: &str, group_id: usize) -> Option<String> {
    let re = Regex::new(regex_str).unwrap();
    if let Some(caps) = re.captures(text) {
        if let Some(matched) = caps.get(group_id) {
            return Some(String::from(matched.as_str()));
        }
    }
    None
}