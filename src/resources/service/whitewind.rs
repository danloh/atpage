use serde::{Deserialize, Serialize};

use crate::{
	helper::utils::str_to_timestamp,
	resources::{
		atpage::AtRecord,
		record::{list_record, uri_parts},
	},
};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct WndListResp {
	pub records: Vec<WndResp>,
	pub cursor: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct WndResp {
	pub uri: String,
	pub cid: String,
	pub value: WndValue,
}

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
	did: &str,
	service: &str,
	cur: Option<String>,
) -> (Vec<AtRecord>, Option<String>) {
	match list_record("com.whtwnd.blog.entry", service, &did, cur).await {
		Ok(raw_data) => match serde_json::from_str::<WndListResp>(&raw_data) {
			Ok(data_res) => {
				let cursor = data_res.cursor;
				let entrylist: Vec<WndResp> = data_res.records;
				let mut recs: Vec<AtRecord> = Vec::new();
				for entry in entrylist {
					let uri = entry.uri;
					let (did, _col, rkey) = uri_parts(&uri);
					let link = format!("https://whtwnd.com/{did}/{rkey}");
					let value = entry.value;

					// to honor the visibility
					let vis = value.visibility.unwrap_or_default();
					if vis != "public" {
						continue;
					}

					let rec = AtRecord {
						kind: String::from("whitewind"),
						title: value.title,
						link,
						content: value.content,
						images: Vec::new(),
						timestamp: str_to_timestamp(&value.createdAt) / 1000,
					};
					recs.push(rec);
				}

				return (recs, cursor);
			}
			Err(e) => {
				gloo::console::log!("get whitewind records error: ", format!("{:?}", e));
				return (Default::default(), None);
			}
		},
		Err(e) => {
			gloo::console::log!("get whitewind records error: ", format!("{:?}", e));
			return (Default::default(), None);
		}
	}
}
