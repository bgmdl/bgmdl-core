use crate::declare::{bangumi, error::CoreError};
use reqwest::header::{ACCEPT, USER_AGENT};

pub type Result<T> = ::std::result::Result<T, CoreError>;

pub async fn search_bangumi(
    client: &reqwest::Client,
    key: String,
) -> Result<bangumi::BangumiSearch> {
    let url = format!("https://api.bgm.tv/search/subject/{}?type=2", key);
    let res = client.get(url.as_str()).send().await?;
    let body = res.text().await?;
    let bangumi_search: bangumi::BangumiSearch = serde_json::from_str(&body)?;
    Ok(bangumi_search)
}

pub async fn get_bangumi_info(
    client: &reqwest::Client,
    key: String,
) -> Result<bangumi::BangumiInfo> {
    let search_result = search_bangumi(client, key).await?;
    let id = search_result.list.unwrap()[0].id.unwrap();
    let url = format!("https://api.bgm.tv/v0/subjects/{}", id);
    let res = client
        .get(url.as_str())
        .header(USER_AGENT, "BGMdl Crawal")
        .header(ACCEPT, "json")
        .send()
        .await?;
    let body = res.text().await?;
    let bangumi_info: bangumi::BangumiInfo = serde_json::from_str(&body)?;
    Ok(bangumi_info)
}

pub async fn get_bangumi_ep(client: &reqwest::Client, bgmid: i32) -> Result<bangumi::BangumiEp> {
    let url = format!(
        "https://api.bgm.tv/v0/episodes?subject_id={}&type=0&limit=200&offset=0",
        bgmid
    );
    let res = client
        .get(url.as_str())
        .header(USER_AGENT, "BGMdl Crawal")
        .header(ACCEPT, "json")
        .send()
        .await?;
    let body = res.text().await?;
    let bangumi_ep: bangumi::BangumiEp = serde_json::from_str(&body)?;
    Ok(bangumi_ep)
}

pub async fn get_bangumi_info_with_bgmid(
    client: &reqwest::Client,
    bgmid: i32,
) -> Result<bangumi::BangumiInfo> {
    let url = format!("https://api.bgm.tv/v0/subjects/{}", bgmid);
    let res = client
        .get(url.as_str())
        .header(USER_AGENT, "BGMdl Crawal")
        .header(ACCEPT, "json")
        .send()
        .await?;
    let body = res.text().await?;
    let bangumi_info: bangumi::BangumiInfo = serde_json::from_str(&body)?;
    Ok(bangumi_info)
}

pub async fn get_bangumi_names(client: &reqwest::Client, key: String) -> Result<Vec<String>> {
    let result = get_bangumi_info(client, key).await?;
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

    #[test]
    fn test_get_bangumi_info() {
        async_run! {
            let client = reqwest::Client::new();
            let res = get_bangumi_info(&client, "前辈是伪娘".to_string()).await;
            dbg!(&res);
        }
    }

    #[test]
    fn test_get_bangumi_names() {
        async_run! {
            let client = reqwest::Client::new();
            let res = get_bangumi_names(&client, "前辈是伪娘".to_string()).await;
            dbg!(&res);
        }
    }
}
