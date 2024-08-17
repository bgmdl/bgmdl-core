use regex::Regex;

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

pub fn parse(title: String) -> ParserTitleResult {
    dbg!(title.clone());
    let re = Regex::new(r"\[([^\n\r\[\]]*)\]").unwrap();
    let mut result = ParserTitleResult {
        fansub: "".to_string(),
        is_number_ep: true,
        ep: 0,
        ep_str: "".to_string(),
        lang: vec![],
        is_multi_lang: false,
        is_multi_ep: false,
        start_ep: 0,
        end_ep: 0
    };
    if let Some(caps) = re.captures(&title.clone()) {
        if let Some(matched) = caps.get(1) {
            result.fansub = String::from(matched.as_str());
        }
    } else {
        dbg!("find 【");
        let re = Regex::new(r"【([^\n\r\[\]]*)】").unwrap();
        if let Some(caps) = re.captures(&title.clone()) {
            if let Some(matched) = caps.get(1) {
                result.fansub = String::from(matched.as_str());
            }
        }
    }
    let re = Regex::new(r"([0-9]{2,4})\s*(-|~|to)\s*(S[0-9]{2,4}E([0-9]{2,4})|E([0-9]{2,4})|([0-9]{2,4}))").unwrap();
    if let Some(caps) = re.captures(&title.clone()) {
        result.is_multi_ep = true;
        result.start_ep = caps.get(1).unwrap().as_str().parse::<u32>().unwrap_or(0);
        result.end_ep = caps.get(4).unwrap_or(caps.get(5).unwrap_or(caps.get(6).unwrap_or(caps.get(3).unwrap()))).as_str().parse::<u32>().unwrap_or(0);
    }
    if title.contains("OVA ") || title.contains("OAD ") || title.contains("SP ") || title.contains("[OVA]") {
        result.is_number_ep = false;
        result.ep = -1;
        let mut ep_str = "".to_string();
        let mut nstr = title.clone();
        let start = title.find("OVA ");
        if let Some(start) = start {
            ep_str = "OVA".to_string();
            nstr = title[start..].to_string();
        }
        let start = title.find("[OVA]");
        if let Some(start) = start {
            ep_str = "OVA".to_string();
            nstr = title[start..].to_string();
        }
        let start = nstr.find("OAD ");
        if let Some(start) = start {
            ep_str = "OAD".to_string();
            nstr = nstr[start..].to_string();
        } 
        let start = nstr.find("SP ");
        if let Some(start) = start {
            ep_str = "SP".to_string();
            nstr = nstr[start..].to_string();
        }
        let mut is_start = false;
        let mut number = 0;
        for c in nstr.chars() {
            if c == ' ' {
                if is_start {
                    ep_str.push(c);
                }
            } else if c.is_numeric() {
                is_start = true;
                number = number * 10 + c.to_digit(10).unwrap() as i32;
            } else {
                if is_start {
                    break;
                }
            }
        }
        if number == 1080 || number == 720 { // error data
            number = 0
        }
        if number != 0 {
            ep_str.push_str(&number.to_string());
        }
        result.ep_str = ep_str;
        
    } else {
        let patterns = vec![
            r"S\d+E(\d+)",     
            r"E(\d+)",         
            r"\b0(\d)\b", 
            r"\b(\d{2})\b",
        ];
        // 判断是否为多集
        for pattern in patterns {
            let re = Regex::new(pattern).unwrap();
            if let Some(caps) = re.captures(&title.clone()) {
                if let Some(matched) = caps.get(1) {
                    if let Ok(num) = matched.as_str().parse::<u32>() {
                        result.ep = num as i32;
                        break;
                    }
                }
            }
        }
    }
    for lang in LANG_LIST.iter() {
        if title.contains(lang) {
            result.lang.push(lang.to_string());
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
        let result = parse(title);
        assert_eq!(result.fansub, String::from("ANi"));
        let title = "【ANi】ABCDEFG - 05".to_string();
        let result = parse(title);
        assert_eq!(result.fansub, String::from("ANi"));
        let title: String = "【喵喵喵】ABCDEFG - 05".to_string();
        let result = parse(title);
        assert_eq!(result.fansub, String::from("喵喵喵"));
    }


    #[test]
    pub fn test_parse_multiep() {
        let title = "[ANi]ABCDEFG - 01~05".to_string();
        let result = parse(title);
        assert_eq!(result.start_ep, 1);
        assert_eq!(result.end_ep, 5);
        let title = "[ANi]ABCDEFG - 01-05".to_string();
        let result = parse(title);
        assert_eq!(result.start_ep, 1);
        assert_eq!(result.end_ep, 5);
        let title = "[ANi]ABCDEFG - 01 - 05".to_string();
        let result = parse(title);
        assert_eq!(result.start_ep, 1);
        assert_eq!(result.end_ep, 5);
        let title = "[ANi]ABCDEFG - S03E01 - S03E05".to_string();
        let result = parse(title);
        assert_eq!(result.start_ep, 1);
        assert_eq!(result.end_ep, 5);
    }
}