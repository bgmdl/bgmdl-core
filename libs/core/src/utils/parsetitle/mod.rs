use regex::Regex;

use crate::utils::regex::regex_get;

#[derive(Debug, Clone)]
pub struct ParserTitleResult {
    pub fansub: String,
    pub is_number_ep: bool,
    pub ep: i32, // ep = -1 when is_number_ep = false
    pub ep_str: String, // = "" when is_number_ep = true
    pub lang: Vec<String>,
    pub is_multi_lang: bool, // if lang > 1 ep can not be trust
    pub start_ep: u32,
    pub end_ep: u32,
    pub is_multi_ep: bool
}

pub const LANG_LIST: [&str; 7] = ["CHS", "CHT", "JPN", "ENG", "RUS", "ITA", "SPA"];

pub fn parse(title: &String) -> ParserTitleResult {
    dbg!(title);
    let regex_fansub_first = r"\[([^\n\r\[\]]*)\]";
    let regex_fansub_second = r"【([^\n\r\[\]]*)】";
    let regex_multi_ep = r"([0-9]{2,4})\s*(-|~|to)\s*(S[0-9]{2,4}E([0-9]{2,4})|E([0-9]{2,4})|([0-9]{2,4}))";
    let regex_special_without_tag = r"(OVA|SP|OAD|SP)( ?([0-9]{1,2})| )";
    let regex_special_with_tag = r"\[(OVA|SP|OAD|SP)\]([\s\S]*([0-9]{1,2})| )";
    let regex_single = [
        r"S\d+E(\d+)",     
        r"E(\d+)",         
        r"\b0(\d)\b", 
        r"\b(\d{2})\b",
    ];

    let mut result = ParserTitleResult {
        fansub: if let Some(fansub) = regex_get(&regex_fansub_first, &title.clone(), 1) {
            String::from(fansub)
        } else {
            if let Some(fansub) = regex_get(&regex_fansub_second, &title.clone(), 1) {
                String::from(fansub)
            } else {
                "".to_string()
            }
        },
        is_number_ep: true,
        ep: 0,
        ep_str: "".to_string(),
        lang: { let mut result = vec![]; 
            for lang in LANG_LIST.iter() {
                if title.contains(lang) {
                    result.push(lang.to_string());
                }
            }
            result
        },
        is_multi_lang: false,
        is_multi_ep: false,
        start_ep: 0,
        end_ep: 0
    };
    let re = Regex::new(&regex_multi_ep).unwrap();
    if let Some(caps) = re.captures(&title.clone()) {
        result.is_multi_ep = true;
        result.start_ep = caps.get(1).unwrap().as_str().parse::<u32>().unwrap_or(0);
        result.end_ep = caps.get(4).unwrap_or(caps.get(5).unwrap_or(caps.get(6).unwrap_or(caps.get(3).unwrap()))).as_str().parse::<u32>().unwrap_or(0);
    }
    let re = Regex::new(&regex_special_without_tag).unwrap();
    if let Some(caps) = re.captures(&title.clone()) {
        result.is_number_ep = false;
        result.ep = -1;
        result.ep_str = caps.get(1).unwrap().as_str().to_string();
        if let Some(ep) = caps.get(3) {
            result.ep = ep.as_str().parse::<i32>().unwrap();
        }
    } else {
        let re = Regex::new(&regex_special_with_tag).unwrap();
        if let Some(caps) = re.captures(&title.clone()) {
            result.is_number_ep = false;
            result.ep = -1;
            result.ep_str = caps.get(1).unwrap().as_str().to_string();
            if let Some(ep) = caps.get(3) {
                result.ep = ep.as_str().parse::<i32>().unwrap();
            }
        } else {
            for pattern in regex_single {
                if let Some(ep) = regex_get(pattern, &title.clone(), 1) {
                    result.is_number_ep = true;
                    result.ep = ep.parse::<i32>().unwrap();
                    break;
                }
            }
        }
    }
    dbg!(result.clone());
    result
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn test_parse_title() {
        let title = "[ANi]ABCDEFG - 05".to_string();
        let result = parse(&title);
        assert_eq!(result.fansub, String::from("ANi"));
        let title = "【ANi】ABCDEFG - 05".to_string();
        let result = parse(&title);
        assert_eq!(result.fansub, String::from("ANi"));
        let title: String = "【喵喵喵】ABCDEFG - 05".to_string();
        let result = parse(&title);
        assert_eq!(result.fansub, String::from("喵喵喵"));
        let title: String = "[ANi] ABCD OVA 1".to_string();
        let result = parse(&title);
        assert_eq!(result.ep_str, String::from("OVA"));
    }


    #[test]
    pub fn test_parse_multiep() {
        let title = "[ANi]ABCDEFG - 01~05".to_string();
        let result = parse(&title);
        assert_eq!(result.start_ep, 1);
        assert_eq!(result.end_ep, 5);
        let title = "[ANi]ABCDEFG - 01-05".to_string();
        let result = parse(&title);
        assert_eq!(result.start_ep, 1);
        assert_eq!(result.end_ep, 5);
        let title = "[ANi]ABCDEFG - 01 - 05".to_string();
        let result = parse(&title);
        assert_eq!(result.start_ep, 1);
        assert_eq!(result.end_ep, 5);
        let title = "[ANi]ABCDEFG - S03E01 - S03E05".to_string();
        let result = parse(&title);
        assert_eq!(result.start_ep, 1);
        assert_eq!(result.end_ep, 5);
    }
}