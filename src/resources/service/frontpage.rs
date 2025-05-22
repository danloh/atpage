use serde::{Deserialize, Serialize};

use crate::{
	helper::utils::str_to_timestamp,
	resources::{
		atpage::AtRecord,
		record::{list_record, uri_parts},
	},
};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FrontpageListResp {
	pub records: Vec<FrontpageResp>,
	pub cursor: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FrontpageResp {
	pub uri: String,
	pub cid: String,
	pub value: FrontpageValue,
}

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
	did: &str,
	service: &str,
	cur: Option<String>,
) -> (Vec<AtRecord>, Option<String>) {
	match list_record("fyi.unravel.frontpage.post", service, &did, cur).await {
		Ok(raw_data) => match serde_json::from_str::<FrontpageListResp>(&raw_data) {
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
						timestamp: str_to_timestamp(&value.createdAt) / 1000,
					};
					recs.push(rec);
				}

				return (recs, cursor);
			}
			Err(e) => {
				gloo::console::log!("get frontpage records error: ", format!("{:?}", e));
				return (Default::default(), None);
			}
		},
		Err(e) => {
			gloo::console::log!("get frontpage records error: ", format!("{:?}", e));
			return (Default::default(), None);
		}
	}
}
