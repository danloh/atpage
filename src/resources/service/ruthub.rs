use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::resources::{
	atpage::AtRecord,
	record::{get_record, list_record},
};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TrackListResp {
	pub records: Vec<TrackResp>,
	pub cursor: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TrackResp {
	pub uri: String,
	pub cid: String,
	pub value: TrackValue,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct TrackValue {
	#[serde(rename = "$type")]
	pub typ: String,
	pub item: String,
	pub status: u8,
	pub note: String,
	pub rating: u8,
	pub collection: Option<String>,
	pub stamp: Option<String>,
	pub updatedAt: i64,
}

#[derive(Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct TrackItem {
	pub kind: String,
	pub item: Item,
	pub note: String,
	pub status: u8,
	pub rating: u8,
	pub collection: Option<String>,
	pub stamp: Option<String>,
	pub updated_at: i64, // timestamp(sec)
}

#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Debug)]
pub struct Item {
	pub kind: String,
	pub uuid: String,
	pub title: String,
	pub cover_image_url: String,
	pub description: String,
	pub attrs: HashMap<String, Vec<String>>,
	pub detail: Option<String>,
	pub skeet: Option<String>,
}

pub async fn get_ruthub_records(
	did: &str,
	serv: &str,
	cursor: Option<String>,
) -> (Vec<AtRecord>, Option<String>) {
	match list_record("com.ruthub.track", serv, did, cursor).await {
		Ok(raw_data) => match serde_json::from_str::<TrackListResp>(&raw_data) {
			Ok(data_res) => {
				let entrylist: Vec<TrackValue> =
					data_res.records.into_iter().map(|r| r.value).collect();

				let mut at_records: Vec<AtRecord> = Vec::new();
				let link = format!("https://ruthub.com/rut/{did}");
				for entry in entrylist {
					let item = get_item_record(&entry.item, did, serv).await;
					if let Some(itm) = item {
						let status = entry.status;
						let act = if status == 1 {
							"TODO"
						} else if status == 2 {
							"DOING"
						} else {
							"DONE"
						};
						let rating = entry.rating;
						let stars = "⭐".repeat((rating as usize / 2).min(5));

						let rec = AtRecord {
							kind: String::from("ruthub"),
							title: format!(
								"[{} - {}] {} {}",
								act,
								itm.kind.to_uppercase(),
								itm.title,
								stars
							),
							link: link.clone(),
							content: format!("{}", entry.note),
							images: vec![itm.cover_image_url],
							timestamp: entry.updatedAt,
						};
						at_records.push(rec);
					}
				}

				return (at_records, data_res.cursor);
			}
			Err(_e) => {
				return Default::default();
			}
		},
		Err(_e) => {
			return Default::default();
		}
	}
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ItemResp {
	pub uri: String,
	pub cid: String,
	pub value: ItemValue,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ItemValue {
	#[serde(rename = "$type")]
	pub typ: String,
	pub kind: String,
	pub uuid: String,
	pub title: String,
	pub cover: String,
	pub description: String,
	pub attrs: HashMap<String, Vec<String>>,
	pub skeet: Option<String>,
}

pub async fn get_item_record(id: &str, did: &str, serv: &str) -> Option<Item> {
	match get_record("com.ruthub.item", id, &serv, did).await {
		Ok(raw_data) => match serde_json::from_str::<ItemResp>(&raw_data) {
			Ok(data_res) => {
				let data = data_res.value;
				let item = Item {
					kind: data.kind,
					uuid: data.uuid,
					title: data.title,
					cover_image_url: data.cover,
					description: data.description,
					attrs: data.attrs,
					detail: None,
					skeet: data.skeet,
				};
				return Some(item);
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
