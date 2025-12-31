//! Ruthub: kanban and write, ...

use serde::{Deserialize, Serialize};

use crate::resources::{atpage::AtRecord, record::list_record};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct EntryListResp {
	pub records: Vec<EntryResp>,
	pub cursor: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct EntryResp {
	pub uri: String,
	pub cid: String,
	pub value: EntryValue,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct EntryValue {
	#[serde(rename = "$type")]
	pub typ: String, // com.ruthub.entry
	pub title: String,
	pub cover: String,
	pub content: String,
	pub visibility: String,
	pub id: i64,
	pub createdAt: i64,
	pub updatedAt: i64,
}

pub async fn get_ruthub_records(
	did: &str,
	serv: &str,
	cursor: Option<String>,
) -> (Vec<AtRecord>, Option<String>) {
	match list_record("com.ruthub.entry", serv, did, cursor).await {
		Ok(raw_data) => match serde_json::from_str::<EntryListResp>(&raw_data) {
			Ok(data_res) => {
				let entrylist: Vec<EntryValue> =
					data_res.records.into_iter().map(|r| r.value).collect();

				let mut at_records: Vec<AtRecord> = Vec::new();

				for entry in entrylist {
					let visibility = entry.visibility;
					if visibility == "public" {
					  let id = entry.id;
						let link = format!("https://ruthub.com/p/{did}/{id}");
						let cover = entry.cover;

						let rec = AtRecord {
							kind: String::from("ruthub"),
							title: entry.title,
							link: link.clone(),
							content: entry.content,
							images: if cover.trim().is_empty() { vec![] } else { vec![cover] },
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
