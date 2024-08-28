use reqwest::{self, blocking, header::{ACCEPT, USER_AGENT}};
use crate::utils::error::GetError;
use crate::declare::bangumi;

pub type Result<T> = ::std::result::Result<T, GetError>;

pub fn search_bangumi(client: blocking::Client, key: String) -> Result<bangumi::BangumiSearch> {
    let url = format!("https://api.bgm.tv/search/subject/{}?type=2", key);
    let res = client.get(url.as_str()).send()?;
    let body = res.text()?;
    let bangumi_search: bangumi::BangumiSearch = serde_json::from_str(&body)?;
    Ok(bangumi_search)
}

pub fn get_bangumi_info(client: blocking::Client, key: String) -> Result<bangumi::BangumiInfo> {
    let search_result = search_bangumi(client.clone(), key)?;
    let id = search_result.list.unwrap()[0].id.unwrap();
    let url = format!("https://api.bgm.tv/v0/subjects/{}", id);
    let res = client.get(url.as_str())
        .header(USER_AGENT, "BGMdl Crawal")
        .header(ACCEPT, "json")
        .send()?;
    let body = res.text()?;
    let bangumi_info: bangumi::BangumiInfo = serde_json::from_str(&body)?;
    Ok(bangumi_info)
}

pub fn get_bangumi_names(client: blocking::Client, key: String) -> Result<Vec<String>> {
    let result = get_bangumi_info(client, key)?;
    let mut names = vec![result.name.unwrap()];
    for keys in result.infobox.unwrap() {
        if keys.key.unwrap() == "别名" {
            let datas = keys.value.unwrap();
            match datas {
                bangumi::BangumiInfoInfoboxValue::TypeA(_) => {}
                bangumi::BangumiInfoInfoboxValue::TypeB(datas) => {
                    for data in datas {
                        names.push(data.v.unwrap());
                    }
                }
            }
        }
    }
    Ok(names)
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::blocking;
    
    #[test]
    fn test_get_bangumi_info() {
        let client = blocking::Client::new();
        let res = get_bangumi_info(client, "前辈是伪娘".to_string());
        panic!("{:?}", res);
    }

    #[test]
    fn test_get_bangumi_names() {
        let client = blocking::Client::new();
        let res = get_bangumi_names(client, "前辈是伪娘".to_string());
        panic!("{:?}", res);
    }
}