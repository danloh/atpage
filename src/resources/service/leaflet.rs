//! leaflet pub
//! lexicons:
//! https://tangled.org/@leaflet.pub/leaflet/blob/main/lexicons/pub/leaflet/document.json
//! https://tangled.org/leaflet.pub/leaflet/blob/main/lexicons/site/standard/document.json

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::{
	helper::utils::str_to_timestamp,
	resources::{atpage::AtRecord, auth::resolve_did, record::{get_record, list_record, uri_parts}},
};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct LeafListResp {
	pub records: Vec<LeafResp>,
	pub cursor: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct LeafResp {
	pub uri: String,
	pub cid: String,
	pub value: LeafValue,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct LeafValue {
	#[serde(rename = "$type")]
	pub kind: String,
	pub title: String,
	pub description: Option<String>,
	pub site: String, // at-uri: to get base_path
	pub publishedAt: String,
	// pub pages: Vec<LinerDocument>, // TODO
}

// TODO, leverage cursor for pagination
pub async fn get_leaflet_records(
	did: &str,
	serv: &str,
	cur: Option<String>,
) -> (Vec<AtRecord>, Option<String>) {
	match list_record("site.standard.document", serv, &did, cur).await {
		Ok(raw_data) => match serde_json::from_str::<LeafListResp>(&raw_data) {
			Ok(data_res) => {
				let cursor = data_res.cursor;
				let entrylist: Vec<LeafResp> = data_res.records;
				let mut recs: Vec<AtRecord> = Vec::new();
				// as a cache to reduce the query base_path of publication(site)
				let mut base_path_map: HashMap<String, String> = HashMap::new();
				for entry in entrylist {
					let value = entry.value;
					let uri = entry.uri;
					let (e_did, _col, rkey) = uri_parts(&uri);
					let publication = value.site;

					let base_path = if let Some(b_path) = base_path_map.get(&publication) {
					  b_path.to_string()
					} else {
  			    let (p_did, _col, p_rkey) = uri_parts(&publication);
  					let p_serv = if p_did == e_did {
  					  serv
  					} else {
  			      &resolve_did(&p_did).await.service
  					};
					  if let Some(val) = get_publication_record(&p_rkey, &p_did, p_serv).await {
  						let url = val.url;
  						base_path_map.insert(publication, url.clone());
  						url
  					} else {
  						continue;
  					}
					};

					let link = if base_path.starts_with("https://") {
						format!("{base_path}/{rkey}")
					} else {
						format!("https://{base_path}/{rkey}")
					};

					let rec = AtRecord {
						kind: String::from("leaflet"),
						title: value.title,
						link,
						content: value.description.unwrap_or_default(),
						images: Vec::new(),
						timestamp: str_to_timestamp(&value.publishedAt) / 1000,
					};
					recs.push(rec);
				}

				return (recs, cursor);
			}
			Err(e) => {
				gloo::console::log!("get leaflet records error: ", format!("{:?}", e));
				return (Default::default(), None);
			}
		},
		Err(e) => {
			gloo::console::log!("get leaflet records error: ", format!("{:?}", e));
			return (Default::default(), None);
		}
	}
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PublicationResp {
	pub uri: String,
	pub cid: String,
	pub value: PublicationValue,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct PublicationValue {
	#[serde(rename = "$type")]
	pub kind: String,
	pub name: String,
	pub url: String,
	pub description: Option<String>,
}

async fn get_publication_record(id: &str, did: &str, serv: &str) -> Option<PublicationValue> {
	match get_record("site.standard.publication", id, &serv, did).await {
		Ok(raw_data) => match serde_json::from_str::<PublicationResp>(&raw_data) {
			Ok(data_res) => {
				let data = data_res.value;

				return Some(data);
			}
			Err(_e) => {
				return None;
			}
		},
		Err(_e) => {
			return None;
		}
	}
}
