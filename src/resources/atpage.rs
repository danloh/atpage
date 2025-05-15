use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::helper::utils::str_to_timestamp;

use super::{auth::{resolve_did, AuthData}, record::{get_record, list_record, uri_parts, PutResp}};

/// for record links
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AtPageResp {
  pub uri: String,
  pub cid: String,
  pub value: AtPageValue,
}

/// for record links
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct AtPageValue {
	#[serde(rename = "$type")]
  pub kind: String,
  pub links: Vec<LinkEntry>,
  pub services: Vec<String>,
  pub title: Option<String>,
  pub description: Option<String>,
  pub style: Option<String>,
}

/// for record link entry
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct LinkEntry {
  pub url: String,
  pub name: String,
  pub order: i32,
  pub createdAt: i64,
  pub description: Option<String>,
  pub icon: Option<String>,
  pub style: Option<String>,
}

pub async fn fetch_atpage(did: String) -> Option<AtPageValue> {
  let service = resolve_did(&did).await.service;
  get_atpage_record(&did, &service).await
}

pub async fn get_atpage_record(did: &str, service: &str) -> Option<AtPageValue> {
	match get_record("one.atpage.page", "self", service, &did).await {
		Ok(raw_data) => {
			match serde_json::from_str::<AtPageResp>(&raw_data) {
				Ok(data_res) => {
					let value: AtPageValue = data_res.value;
					return Some(value);
				}
				Err(e) => {
          gloo::console::log!("get_atpage_record de error: ", format!("{:?}", e));
				  return None;
				}
			}
		}
	  Err(e) => {
      gloo::console::log!("get_atpage_record error: ", format!("{:?}", e));
			return None;
		}
	}
}

pub async fn put_atpage_record(
	usr: AuthData,
	data: &AtPageValue,
) -> Result<PutResp, String> {
  let post_uri = format!("{}/xrpc/com.atproto.repo.putRecord", usr.service);

	let post = json!({
		"repo": &usr.did,
		"collection": "one.atpage.page",
		"rkey": "self",
		"record": {
			"$type": "one.atpage.page",
      "links": data.links,
      "services": data.services,
      "title": data.title.clone().unwrap_or_default(),
      "description": data.description.clone().unwrap_or_default(),
      "style": data.style.clone().unwrap_or_default(),
		},
		"validate": false
	});

  let client = reqwest::Client::new();
  let res = client
    .post(post_uri)
		.json(&post)
		.header("Authorization", format!("Bearer {}", &usr.access))
    .send()
    .await
    .map_err(|e| format!("put_atpage_record request err: {:?}", e))?
    .text()
    .await
    .map_err(|e| format!("put_atpage_record err: {:?}", e))?;

  serde_json::from_str::<PutResp>(&res)
    .map_err(|e| format!("put_atpage_record de err: {:?}", e))
}


// ===================================================================================
// ===========  Service Record =======================================================
// ===================================================================================

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AtRecord {
  pub kind: String,
  pub title: String,
  pub link: String,
  pub content: String,
  pub images: Vec<String>,
  pub timestamp: i64,
}

#[derive(Clone, Default)]
pub struct RecordsRes {
  pub records: Vec<AtRecord>,
  pub cursor: Option<String>,
}

pub async fn fetch_records((did, services): (String, Vec<String>)) -> RecordsRes {
  let serv = resolve_did(&did).await.service;
  let mut vec = Vec::new();
  for service in services {
    if service == "frontpage" {
      let (mut frontpages, _f_cur) = get_frontpage_records(&did, &serv, None).await;
      vec.append(&mut frontpages);
    }
    if service == "whitewind" {
      let (mut wnds, _w_cur) = get_wnd_records(&did, &serv, None).await; 
      vec.append(&mut wnds);
    }
    if service == "pinksea" {
      let (mut pinkseas, _p_cur) = get_pinksea_records(&did, &serv, None).await; 
      vec.append(&mut pinkseas);
    }
  }
  
  let final_vec = vec
    .into_iter()
    .sorted_by(|a, b| Ord::cmp(&b.timestamp, &a.timestamp))
    .collect();

  RecordsRes {
    records: final_vec,
    cursor: None,
  }
}

/// whitewind: blog service
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct WndListResp {
  pub records: Vec<WndResp>,
  pub cursor: Option<String>,
}

/// for record blog entry
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct WndResp {
  pub uri: String,
  pub cid: String,
  pub value: WndValue,
}

/// for record blog entry
#[derive(Clone, Default, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct WndValue {
	#[serde(rename = "$type")]
  pub kind: String,
  pub title: String,
  pub content: String,
  pub createdAt: String,
  pub visibility: Option<String>,
}

// TODO, leverage cursor for pagination 
pub async fn get_wnd_records(
  did: &str, service: &str, cur: Option<String>
) -> (Vec<AtRecord>, Option<String>) {
	match list_record("com.whtwnd.blog.entry", service, &did, cur).await {
		Ok(raw_data) => {
			match serde_json::from_str::<WndListResp>(&raw_data) {
				Ok(data_res) => {
          let cursor = data_res.cursor;
					let entrylist: Vec<WndResp> = data_res.records;
          let mut recs: Vec<AtRecord> = Vec::new();
          for entry in entrylist {
            let uri = entry.uri;
            let (did, _col, rkey) = uri_parts(&uri);
            let link = format!("https://whtwnd.com/{did}/{rkey}");
            let value = entry.value;
            let rec = AtRecord {
              kind: String::from("whitewind"),
              title: value.title,
              link,
              content: value.content,
              images: Vec::new(),
              timestamp: str_to_timestamp(&value.createdAt),
            };
            recs.push(rec);
          }

					return (recs, cursor); 
        }
				Err(_e) => {
				  return (Default::default(), None);
				}
			}
		}
	  Err(_e) => {
			return (Default::default(), None);
		}
	}
}

/// frontpage: link share
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FrontpageListResp {
  pub records: Vec<FrontpageResp>,
  pub cursor: Option<String>,
}

/// for record frontpage: link share
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FrontpageResp {
  pub uri: String,
  pub cid: String,
  pub value: FrontpageValue,
}

/// for record frontpage.fyi
#[derive(Clone, Default, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct FrontpageValue {
	#[serde(rename = "$type")]
  pub kind: String,
  pub title: String,
  pub url: String,
  pub createdAt: String,
}

// TODO, leverage cursor for pagination 
pub async fn get_frontpage_records(
  did: &str, service: &str, cur: Option<String>
) -> (Vec<AtRecord>, Option<String>) {
	match list_record("fyi.unravel.frontpage.post", service, &did, cur).await {
		Ok(raw_data) => {
			match serde_json::from_str::<FrontpageListResp>(&raw_data) {
				Ok(data_res) => {
          let cursor = data_res.cursor;
					let entrylist: Vec<FrontpageResp> = data_res.records;
          let mut recs: Vec<AtRecord> = Vec::new();
          for entry in entrylist {
            let uri = entry.uri;
            let (did, _col, rkey) = uri_parts(&uri);
            let link = format!("https://frontpage.fyi/post/{did}/{rkey}");
            let value = entry.value;
            let rec = AtRecord {
              kind: String::from("frontpage"),
              title: value.title.clone(),
              link,
              content: format!("{} <{}>", value.title, value.url),
              images: Vec::new(),
              timestamp: str_to_timestamp(&value.createdAt),
            };
            recs.push(rec);
          }

					return (recs, cursor); 
        }
				Err(_e) => {
				  return (Default::default(), None);
				}
			}
		}
	  Err(_e) => {
			return (Default::default(), None);
		}
	}
}

/// frontpage: link share
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PinkSeaListResp {
  pub records: Vec<PinkSeaResp>,
  pub cursor: Option<String>,
}

/// for record frontpage: link share
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PinkSeaResp {
  pub uri: String,
  pub cid: String,
  pub value: PinkSeaValue,
}

/// for record frontpage.fyi
#[derive(Clone, Default, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct PinkSeaValue {
	#[serde(rename = "$type")]
  pub kind: String,
  pub image: PinkSeaBlob,
  pub nsfw: bool,
  pub createdAt: String,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct PinkSeaBlob {
	#[serde(rename = "$type")]
  pub kind: String,
  #[serde(rename = "$ref")]
  pub linkref: PinkSeaBlobLink,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct PinkSeaBlobLink {
	#[serde(rename = "$link")]
  pub link: String,
}

// TODO, leverage cursor for pagination 
pub async fn get_pinksea_records(
  did: &str, service: &str, cur: Option<String>
) -> (Vec<AtRecord>, Option<String>) {
	match list_record("com.shinolabs.pinksea.oekaki", service, &did, cur).await {
		Ok(raw_data) => {
			match serde_json::from_str::<PinkSeaListResp>(&raw_data) {
				Ok(data_res) => {
          let cursor = data_res.cursor;
					let entrylist: Vec<PinkSeaResp> = data_res.records;
          let mut recs: Vec<AtRecord> = Vec::new();
          for entry in entrylist {
            let uri = entry.uri;
            let (did, _col, rkey) = uri_parts(&uri);
            let link = format!("https://pinksea.art/{did}/oekaki/{rkey}");
            let value = entry.value;
            let img_link = format!(
              "https://harbor.pinksea.art/{did}/{}", 
              value.image.linkref.link
            );
            let rec = AtRecord {
              kind: String::from("pinksea"),
              title: String::new(),
              link,
              content: String::new(),
              images: vec![img_link],
              timestamp: str_to_timestamp(&value.createdAt),
            };
            recs.push(rec);
          }

					return (recs, cursor); 
        }
				Err(_e) => {
				  return (Default::default(), None);
				}
			}
		}
	  Err(_e) => {
			return (Default::default(), None);
		}
	}
}
