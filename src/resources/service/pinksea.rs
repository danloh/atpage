use serde::{Deserialize, Serialize};

use crate::{
	helper::utils::str_to_timestamp,
	resources::{
		atpage::AtRecord,
		record::{list_record, uri_parts},
	},
};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PinkSeaListResp {
	pub records: Vec<PinkSeaResp>,
	pub cursor: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PinkSeaResp {
	pub uri: String,
	pub cid: String,
	pub value: PinkSeaValue,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct PinkSeaValue {
	#[serde(rename = "$type")]
	pub kind: String,
	pub image: PinkSeaImage,
	pub nsfw: bool,
	pub createdAt: String,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PinkSeaImage {
	pub blob: PinkSeaBlob,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PinkSeaBlob {
	#[serde(rename = "ref")]
	pub linkref: PinkSeaBlobLink,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PinkSeaBlobLink {
	#[serde(rename = "$link")]
	pub link: String,
}

// TODO, leverage cursor for pagination
pub async fn get_pinksea_records(
	did: &str,
	service: &str,
	cur: Option<String>,
) -> (Vec<AtRecord>, Option<String>) {
	match list_record("com.shinolabs.pinksea.oekaki", service, &did, cur).await {
		Ok(raw_data) => {
			// gloo::console::log!("pinksea records: ", format!("{:?}", raw_data));
			match serde_json::from_str::<PinkSeaListResp>(&raw_data) {
				Ok(data_res) => {
					let cursor = data_res.cursor;
					let entrylist: Vec<PinkSeaResp> = data_res.records;
					let mut at_records: Vec<AtRecord> = Vec::new();
					for entry in entrylist {
						let uri = entry.uri;
						let (did, _col, rkey) = uri_parts(&uri);
						let link = format!("https://pinksea.art/{did}/oekaki/{rkey}");
						let value = entry.value;
						
						// to honor the nsfw
						if value.nsfw {
							continue;
						}
						
						let img_link = format!(
							"https://harbor.pinksea.art/{did}/{}",
							value.image.blob.linkref.link
						);
						let rec = AtRecord {
							kind: String::from("pinksea"),
							title: String::new(),
							link,
							content: String::new(),
							images: vec![img_link],
							timestamp: str_to_timestamp(&value.createdAt) / 1000,
						};
						at_records.push(rec);
					}

					// gloo::console::log!("pinksea res: ", format!("{:?}", at_records));

					return (at_records, cursor);
				}
				Err(e) => {
					gloo::console::log!("get pinksea records error: ", format!("{:?}", e));
					return (Default::default(), None);
				}
			}
		}
		Err(e) => {
			gloo::console::log!("get pinksea records error: ", format!("{:?}", e));
			return (Default::default(), None);
		}
	}
}
