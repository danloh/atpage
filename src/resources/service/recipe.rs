//! recipe exchange
//! lexicons: 
//! https://recipe.exchange/lexicons/recipe.json

use serde::{Deserialize, Serialize};

use crate::{
	helper::utils::str_to_timestamp,
	resources::{
		atpage::AtRecord,
		record::{list_record, uri_parts},
	},
};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RecipeListResp {
	pub records: Vec<RecipeResp>,
	pub cursor: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RecipeResp {
	pub uri: String,
	pub cid: String,
	pub value: RecipeValue,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct RecipeValue {
	#[serde(rename = "$type")]
	pub kind: String,
	pub name: String,
	pub text: String,
	// pub ingredients: Vec<String>,
	// pub instructions: Vec<String>,
	// pub createdAt: String,
	pub updatedAt: String,
	pub embed: Option<RecipeEmbed>,
	// pub recipeCategory: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RecipeEmbed {
	#[serde(rename = "$type")]
	pub kind: String,
	pub images: Vec<RecipeImage>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RecipeImage {
	pub alt: String,
	pub image: RecipeBlob,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RecipeBlob {
	#[serde(rename = "$type")]
	pub kind: String,
	#[serde(rename = "ref")]
	pub imgref: RecipeImgLink,
	// pub mimeType: String,
  // size: i32,
	// aspectRatio: {width: i32, height: i32}
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RecipeImgLink {
	#[serde(rename = "$link")]
	pub link: String,
}

// TODO, leverage cursor for pagination
pub async fn get_recipe_records(
	did: &str,
	serv: &str,
	cur: Option<String>,
) -> (Vec<AtRecord>, Option<String>) {
	match list_record("exchange.recipe.recipe", serv, &did, cur).await {
		Ok(raw_data) => {
			// gloo::console::log!("recipe records: ", format!("{:?}", raw_data));
			match serde_json::from_str::<RecipeListResp>(&raw_data) {
				Ok(data_res) => {
					let cursor = data_res.cursor;
					let entrylist: Vec<RecipeResp> = data_res.records;
					let mut at_records: Vec<AtRecord> = Vec::new();
					for entry in entrylist {
						let uri = entry.uri;
						let (did, _col, rkey) = uri_parts(&uri);
						let link = format!("https://recipe.exchange/recipes/{rkey}");

						let value = entry.value; 
						let mut imgs_txt = String::new();
						
						let imgs = value.embed.map(|emb| {
							emb.images.into_iter().map(|img| {
								let alt = img.alt;
								// let mime = img.image.mimeType;
								let lnk = img.image.imgref.link;
								(format!("https://cdn.bsky.app/img/feed_thumbnail/plain/{did}/{lnk}"), alt)
							})
							.collect::<Vec<(String, String)>>()
						})
						.unwrap_or_default();

						for img in imgs {
							imgs_txt.push_str(&format!("![{}]({})  ", img.1, img.0));
						}

						let rec = AtRecord {
							kind: String::from("recipe"),
							title: value.name,
							link,
							content: format!("{} {}", imgs_txt, value.text),
							images: vec![],
							timestamp: str_to_timestamp(&value.updatedAt) / 1000,
						};
						at_records.push(rec);
					}

					// gloo::console::log!("recipe res: ", format!("{:?}", at_records));

					return (at_records, cursor);
				}
				Err(e) => {
					gloo::console::log!("get recipe records error: ", format!("{:?}", e));
					return (Default::default(), None);
				}
			}
		}
		Err(e) => {
			gloo::console::log!("get recipe records error: ", format!("{:?}", e));
			return (Default::default(), None);
		}
	}
}
