use serde::{Deserialize, Serialize};

use crate::{
  helper::utils::str_to_timestamp, 
  resources::{atpage::AtRecord, record::{list_record, uri_parts}}
};

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
