//! photo-sharing
//! lexicons: 
//! https://tangled.sh/@grain.social/grain/blob/main/lexicons/social/grain/favorite.json 
//! https://tangled.sh/@grain.social/grain/blob/main/lexicons/social/grain/gallery/gallery.json

use serde::{Deserialize, Serialize};

use crate::{
	helper::utils::str_to_timestamp,
	resources::{
		atpage::AtRecord, auth::resolve_did, record::{get_record, list_record, uri_parts}
	},
};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GalleryListResp {
	pub records: Vec<GalleryResp>,
	pub cursor: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GalleryResp {
	pub uri: String,
	pub cid: String,
	pub value: GalleryValue,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct GalleryValue {
	#[serde(rename = "$type")]
	pub kind: String,
	pub title: String,
	pub description: Option<String>,
	pub createdAt: String,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FavListResp {
	pub records: Vec<FavResp>,
	pub cursor: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FavResp {
	pub uri: String,
	pub cid: String,
	pub value: FavValue,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct FavValue {
	#[serde(rename = "$type")]
	pub kind: String,
	pub subject: String,
	pub createdAt: String,
}

// TODO, leverage cursor for pagination
pub async fn get_grain_records(
	did: &str,
	serv: &str,
	_cur: Option<String>,
) -> (Vec<AtRecord>, Option<String>) {
	let mut at_records: Vec<AtRecord> = Vec::new();

	match list_record("social.grain.gallery", serv, &did, None).await {
		Ok(raw_data) => {
			// gloo::console::log!("grain gallery records: ", format!("{:?}", raw_data));
			match serde_json::from_str::<GalleryListResp>(&raw_data) {
				Ok(data_res) => {
					// let cursor = data_res.cursor;
					let entrylist: Vec<GalleryResp> = data_res.records;
					for entry in entrylist {
						let uri = entry.uri;
						let (did, _col, rkey) = uri_parts(&uri);
						let link = format!("https://grain.social/profile/{did}/gallery/{rkey}");
						let value = entry.value;
						
						let rec = AtRecord {
							kind: String::from("grain"),
							title: format!("Created Gallery: {}", value.title),
							link,
							content: value.description.unwrap_or_default(),
							images: vec![],
							timestamp: str_to_timestamp(&value.createdAt) / 1000,
						};
						at_records.push(rec);
					}
				}
				Err(e) => {
					gloo::console::log!("get grain gallery records error: ", format!("{:?}", e));
				}
			}
		}
		Err(e) => {
			gloo::console::log!("get grain gallery records error: ", format!("{:?}", e));
		}
	}
	
	match list_record("social.grain.favorite", serv, &did, None).await {
		Ok(raw_data) => {
			// gloo::console::log!("grain favorite records: ", format!("{:?}", raw_data));
			match serde_json::from_str::<FavListResp>(&raw_data) {
				Ok(data_res) => {
					// let cursor = data_res.cursor;
					let entrylist: Vec<FavResp> = data_res.records;
					for entry in entrylist {
						let value = entry.value;
						let subject = value.subject;
            let (g_did, col, g_rkey) = uri_parts(&subject);
						if col == "social.grain.gallery" {
							let sv = resolve_did(&g_did).await.service;
							if let Some(gallery) = get_gallery_record(&g_rkey, &g_did, &sv).await {
								let link = format!("https://grain.social/profile/{g_did}/gallery/{g_rkey}");
								let rec = AtRecord {
									kind: String::from("grain"),
									title: format!("Favorited Gallery: {}", gallery.title),
									link,
									content: gallery.description.unwrap_or_default(),
									images: vec![],
									timestamp: str_to_timestamp(&gallery.createdAt) / 1000,
								};
								at_records.push(rec);
							}
						}
					}
				}
				Err(e) => {
					gloo::console::log!("get grain favorite records error: ", format!("{:?}", e));
				}
			}
		}
		Err(e) => {
			gloo::console::log!("get grain favorite records error: ", format!("{:?}", e));
		}
	}

	// gloo::console::log!("grain res: ", format!("{:?}", at_records));

	(at_records, None)
}

async fn get_gallery_record(rkey: &str, did: &str, serv: &str) -> Option<GalleryValue> {
	match get_record("social.grain.gallery", rkey, &serv, did).await {
		Ok(raw_data) => match serde_json::from_str::<GalleryResp>(&raw_data) {
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
