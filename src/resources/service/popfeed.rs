//! Review book, movie, ... 

use serde::{Deserialize, Serialize};

use crate::{helper::utils::str_to_timestamp, resources::{
	atpage::AtRecord, auth::resolve_did, record::{get_record, list_record, uri_parts}
}};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ListItemsResp {
	pub records: Vec<ListItemResp>,
	pub cursor: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ListItemResp {
	pub uri: String,
	pub cid: String,
	pub value: ListItemValue,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ListItemValue {
	#[serde(rename = "$type")] // social.popfeed.feed.listItem
	pub typ: String, 
	pub addedAt: String,
	pub listUri: String, // at-uri
	pub creativeWorkType: String,
	pub title: Option<String>,
	pub listType: Option<String>,
	pub posterUrl: Option<String>,
	pub backdropUrl: Option<String>,
	pub description: Option<String>,
	// pub genres: Vec<String>,
	// pub mainCredit: String,
	// pub releaseDate: String,
	// pub mainCreditRole: String,
	// pub identifiers: Object,
}

// #[derive(Clone, Default, Serialize, Deserialize)]
// pub struct ListsResp {
// 	pub records: Vec<ListResp>,
// 	pub cursor: Option<String>,
// }

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ListResp {
	pub uri: String,
	pub cid: String,
	pub value: ListValue,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ListValue {
	#[serde(rename = "$type")] // social.popfeed.feed.list
	pub typ: String, 
	pub name: String,
	pub createdAt: String,
	pub listType: String,
	pub description: String,
	// pub ordered: Option<bool>,
	// pub authorDid: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct LikesResp {
	pub records: Vec<LikeResp>,
	pub cursor: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct LikeResp {
	pub uri: String,
	pub cid: String,
	pub value: LikeValue,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct LikeValue {
	#[serde(rename = "$type")] // social.popfeed.feed.like
	pub typ: String, 
	pub createdAt: String,
	pub subjectUri: String, // at-uri
	pub subjectType: String,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct NotesResp {
	pub records: Vec<NoteResp>,
	pub cursor: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct NoteResp {
	pub uri: String,
	pub cid: String,
	pub value: NoteValue,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct NoteValue {
	#[serde(rename = "$type")] // social.popfeed.feed.note
	pub typ: String, 
	pub title: String,
	pub text: String,
	pub posterUrl: String,
	pub backdropUrl: String,
	pub createdAt: String,
	pub releaseDate: String,
	pub creativeWorkType: String,
	// pub facets: Vec<String>,
	// pub identifiers: Object,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ReviewsResp {
	pub records: Vec<ReviewResp>,
	pub cursor: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ReviewResp {
	pub uri: String,
	pub cid: String,
	pub value: ReviewValue,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ReviewValue {
	#[serde(rename = "$type")] // social.popfeed.feed.review
	pub typ: String, 
	pub title: String,
	pub text: String,
	pub containsSpoilers: bool,
	pub createdAt: String,
	pub creativeWorkType: String,
	pub rating: usize,
	// pub facets: Vec<String>,
	// pub tags: Vec<String>,
	// pub genres: Vec<String>,
	// pub mainCredit: String,
	// pub releaseDate: String,
	// pub identifiers: Object,
	// pub isRevisit: bool,
}

pub async fn get_popfeed_records(
	did: &str,
	serv: &str,
	cursor: Option<String>,
) -> (Vec<AtRecord>, Option<String>) {
	let mut at_records: Vec<AtRecord> = Vec::new();
	// get reviews 
	if let Ok(rw) = list_record("social.popfeed.feed.review", serv, did, cursor.clone()).await {
		if let Ok(res) = serde_json::from_str::<ReviewsResp>(&rw) {
			for rec in res.records {
				let at_uri = rec.uri;
				let link = format!("https://popfeed.social/review/{at_uri}");
				let entry = rec.value;
				let rating = entry.rating;
				let stars = "⭐".repeat((rating / 2).min(5));
				
				let rec = AtRecord {
					kind: String::from("popfeed"),
					title: format!("Review: {} [{}] {}", entry.title, entry.creativeWorkType, stars),
					link: link.clone(),
					content: entry.text,
					images: vec![],
					timestamp: str_to_timestamp(&entry.createdAt) / 1000,
				};
				at_records.push(rec);
			}
		}
	}

	// get notes 
	if let Ok(n_data) = list_record("social.popfeed.feed.note", serv, did, cursor.clone()).await {
		if let Ok(res) = serde_json::from_str::<NotesResp>(&n_data) {
			for rec in res.records {
				let at_uri = rec.uri;
				let link = format!("https://popfeed.social/notes/{at_uri}");
				let entry = rec.value;

				let rec = AtRecord {
					kind: String::from("popfeed"),
					title: format!("Note on {}", entry.title),
					link: link.clone(),
					content: entry.text,
					images: vec![],
					timestamp: str_to_timestamp(&entry.createdAt) / 1000,
				};
				at_records.push(rec);
			}
		}
	}

	// get likes 
	if let Ok(lk_data) = list_record("social.popfeed.feed.like", serv, did, cursor.clone()).await {
		if let Ok(res) = serde_json::from_str::<LikesResp>(&lk_data) {
			for rec in res.records {
				let entry = rec.value;
				
				let sub_typ = entry.subjectType;
				let sub_uri = entry.subjectUri;
				let (s_did, s_col, s_rkey) = uri_parts(&sub_uri);
				let s_serv = if did == s_did {
					serv.to_string()
				} else { 
					resolve_did(&s_did).await.service
				};

				let (title, content, link) = if sub_typ == "list" {
					if let Some(lst) = get_list_record(&s_rkey, &s_did, &s_col, &s_serv).await {
						(lst.name, lst.description, format!("https://popfeed.social/list/{}", sub_uri))
					} else {
						continue;
					}
				} else if sub_typ == "review" {
					if let Some(rv) = get_review_record(&s_rkey, &s_did, &s_serv).await {
						(rv.title, rv.text, format!("https://popfeed.social/review/{}", sub_uri))
					} else {
						continue;
					}
				} else if sub_typ == "note" {
					if let Some(nt) = get_note_record(&s_rkey, &s_did, &s_serv).await {
						(nt.title, nt.text, format!("https://popfeed.social/notes/{}", sub_uri))
					} else {
						continue;
					}
				} else {
					continue;
				};

				let rec = AtRecord {
					kind: String::from("popfeed"),
					title: format!("Like {}: {}", sub_typ, title),
					link,
					content,
					images: vec![],
					timestamp: str_to_timestamp(&entry.createdAt) / 1000,
				};
				at_records.push(rec);
			}
		}
	}

	// get listitems 
	if let Ok(lk_data) = list_record("social.popfeed.feed.listItem", serv, did, cursor).await {
		if let Ok(res) = serde_json::from_str::<ListItemsResp>(&lk_data) {
			for rec in res.records {
				let entry = rec.value;
				
				let sub_typ = entry.creativeWorkType;
				let lst_uri = entry.listUri;
				let (l_did, l_col, l_rkey) = uri_parts(&lst_uri);
				let l_serv = if did == l_did {
					serv.to_string()
				} else { 
					resolve_did(&l_did).await.service
				};

				let link = format!("https://popfeed.social/list/{}", lst_uri);
				let item_title = entry.title.unwrap_or_default();
				let item_img = entry.posterUrl.unwrap_or_default();
				let images = if item_img.trim().is_empty() {
					vec![]
				} else {
					vec![item_img]
				};

				let (title, content) = 
				  if let Some(lst) = get_list_record(&l_rkey, &l_did, &l_col, &l_serv).await {
						(lst.name, lst.description)
					} else {
						continue;
					};

				let rec = AtRecord {
					kind: String::from("popfeed"),
					title: format!("Add [{}] {} to list: {}", sub_typ, item_title, title),
					link,
					content,
					images,
					timestamp: str_to_timestamp(&entry.addedAt) / 1000,
				};
				at_records.push(rec);
			}
		}
	}

	return (at_records, None);
}

async fn get_list_record(id: &str, did: &str, col: &str, serv: &str) -> Option<ListValue> {
	match get_record(col, id, &serv, did).await {
		Ok(raw_data) => match serde_json::from_str::<ListResp>(&raw_data) {
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

async fn get_note_record(id: &str, did: &str, serv: &str) -> Option<NoteValue> {
	match get_record("social.popfeed.feed.note", id, &serv, did).await {
		Ok(raw_data) => match serde_json::from_str::<NoteResp>(&raw_data) {
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

async fn get_review_record(id: &str, did: &str, serv: &str) -> Option<ReviewValue> {
	match get_record("social.popfeed.feed.review", id, &serv, did).await {
		Ok(raw_data) => match serde_json::from_str::<ReviewResp>(&raw_data) {
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
