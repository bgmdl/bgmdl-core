use nyaa_si::{model::Torrent, Client, NyaaCategory, NyaaClient, QueryBuilder, Sort};
use std::collections::HashMap;

use crate::utils::parsetitle;

pub async fn query_on_nyaa(key: &String) -> Vec<Torrent> {
    let query = QueryBuilder::new()
        .search((*key).as_str())
        .sort(Sort::Downloads)
        .category(NyaaCategory::Anime)
        .build();
    let client = NyaaClient::new();
    client.get(&query).await.unwrap()
}

#[derive(Debug, Clone)]
pub struct FanSubInfo {
    pub name: String,
    pub lang: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub fansub_list: Vec<String>,
    pub fansub_map: HashMap<String, u32>,
    pub fansub_info: HashMap<String, FanSubInfo>,
}

//search by name list, get fansubs data.
pub async fn search_by_namelist(name_list: &Vec<String>) -> SearchResult {
    let mut fansub_list = vec![];
    let mut fansub_map: HashMap<String, u32> = HashMap::new();
    let mut fansub_info: HashMap<String, FanSubInfo> = HashMap::new();
    let name_list = &name_list[..];
    for name in name_list {
        let result = query_on_nyaa(&name).await;
        for torrent in result {
            let data = parsetitle::parse(&torrent.title);
            if fansub_map.contains_key(&data.fansub) {
                let count = fansub_map.get(&data.fansub).unwrap();
                if data.ep > *count as i32 {
                    fansub_map.insert(data.fansub.clone(), data.ep as u32);
                }
                let mut info = fansub_info.get(&data.fansub).unwrap().clone();
                for langs in &data.lang {
                    if !info.lang.contains(langs) {
                        info.lang.push(langs.clone());
                    }
                }
                fansub_info.insert(data.fansub.clone(), info);
            } else {
                fansub_list.push(data.fansub.clone());
                fansub_map.insert(data.fansub.clone(), data.ep as u32);
                let info = FanSubInfo {
                    name: data.fansub.clone(),
                    lang: data.lang.clone(),
                };
                fansub_info.insert(data.fansub.clone(), info);
            }
        }
    }
    dbg!(&fansub_list);
    dbg!(&fansub_map);
    dbg!(&fansub_info);
    SearchResult {
        fansub_list,
        fansub_map,
        fansub_info,
    }
}

pub async fn search_by_fansub_with_name(fansub: &String, name: &String) -> Vec<Torrent> {
    let query = QueryBuilder::new()
        .search(format!("{} {}", *fansub, *name).as_str())
        .sort(Sort::Downloads)
        .category(NyaaCategory::Anime)
        .build();
    let client = NyaaClient::new();
    let result = client.get(&query).await.unwrap();
    let mut torrent_list = vec![];
    for torrent in result {
        let data = parsetitle::parse(&torrent.title);
        if data.fansub == (*fansub) {
            torrent_list.push(torrent);
        }
    }
    dbg!(&torrent_list);
    torrent_list
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_query_on_nyaa() {
        let result = async_run! {
            query_on_nyaa(&"Senpai wa Otokonoko".to_string()).await
        };
        assert!(result.len() > 0);
    }

    #[test]
    fn test_search_by_namelist() {
        let name_list = vec!["Senpai wa Otokonoko 1080p".to_string()];
        async_run!{
            search_by_namelist(&name_list).await
        };
        assert!(true);
    }

    #[test]
    fn test_search_by_fansub_with_name() {
        let fansub = "ANi".to_string();
        let name = "Senpai wa Otokonoko".to_string();
        let result = async_run! {
            search_by_fansub_with_name(&fansub, &name).await
        };
        assert!(result.len() > 0);
    }
}
