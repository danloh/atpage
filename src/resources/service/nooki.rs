//! nooki.me: reddit-alternative 

use serde::{Deserialize, Serialize};

use crate::{helper::utils::str_to_timestamp, resources::{
	atpage::AtRecord, auth::resolve_did, record::{get_record, list_record, uri_parts}
}};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PostsResp {
	pub records: Vec<PostResp>,
	pub cursor: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PostResp {
	pub uri: String,
	pub cid: String,
	pub value: PostValue,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct PostValue {
	#[serde(rename = "$type")] // community.nooki.posts
	pub typ: String, 
	pub title: String,
	pub content: String, 
	pub createdAt: String, 
	pub communityUrl: String, // at-uri
	pub communityName: String,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CommentsResp {
	pub records: Vec<CommentResp>,
	pub cursor: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CommentResp {
	pub uri: String,
	pub cid: String,
	pub value: CommentValue,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct CommentValue {
	#[serde(rename = "$type")] // community.nooki.comments
	pub typ: String, 
	pub content: String, 
	pub postUri: String, // at-uri
	pub createdAt: String,
	pub parentUri: String, // at-uri
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpvotesResp {
	pub records: Vec<UpvoteResp>,
	pub cursor: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpvoteResp {
	pub uri: String,
	pub cid: String,
	pub value: UpvoteValue,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct UpvoteValue {
	#[serde(rename = "$type")] // community.nooki.upvotes
	pub typ: String, 
	pub postUri: String, // at-uri
	pub createdAt: String,
}

pub async fn get_nooki_records(
	did: &str,
	serv: &str,
	cursor: Option<String>,
) -> (Vec<AtRecord>, Option<String>) {
	let mut at_records: Vec<AtRecord> = Vec::new();
	// get posts 
	if let Ok(pst) = list_record("community.nooki.posts", serv, did, cursor.clone()).await {
		if let Ok(res) = serde_json::from_str::<PostsResp>(&pst) {
			for rec in res.records {
				let entry = rec.value;
				let title = entry.title;
				let title_slug = slug::slugify(&bleach_slug(&title));
				let link = format!("https://nooki.me/post/{title_slug}");
				
				let rec = AtRecord {
					kind: String::from("nooki"),
					title,
					link,
					content: entry.content,
					images: vec![],
					timestamp: str_to_timestamp(&entry.createdAt) / 1000,
				};
				at_records.push(rec);
			}
		}
	}

	// get comments 
	if let Ok(cmt_data) = list_record("community.nooki.comments", serv, did, cursor.clone()).await {
		if let Ok(res) = serde_json::from_str::<CommentsResp>(&cmt_data) {
			for rec in res.records {
				let entry = rec.value;
				let post_uri = entry.postUri;
				let (p_did, _p_col, p_rkey) = uri_parts(&post_uri);
				let p_serv = if did == p_did {
					serv.to_string()
				} else { 
					resolve_did(&p_did).await.service
				};
				let (title, link) = if let Some(pst) = get_post_record(&p_rkey, &p_did, &p_serv).await {
					let title = pst.title;
          let p_title_slug = slug::slugify(&bleach_slug(&title));
				  let link = format!("https://nooki.me/post/{p_title_slug}");
					(title, link)
				} else {
					(post_uri, String::new())
				};

				let rec = AtRecord {
					kind: String::from("nooki"),
					title: format!("Comment on {}", title),
					link,
					content: entry.content,
					images: vec![],
					timestamp: str_to_timestamp(&entry.createdAt) / 1000,
				};
				at_records.push(rec);
			}
		}
	}

	// get upvotes 
	if let Ok(up_data) = list_record("community.nooki.upvotes", serv, did, cursor.clone()).await {
		if let Ok(res) = serde_json::from_str::<UpvotesResp>(&up_data) {
			for rec in res.records {
				let entry = rec.value;
				let post_uri = entry.postUri;
				let (p_did, _p_col, p_rkey) = uri_parts(&post_uri);
				let p_serv = if did == p_did {
					serv.to_string()
				} else { 
					resolve_did(&p_did).await.service
				};
				let (title, link, content) = if let Some(pst) = get_post_record(&p_rkey, &p_did, &p_serv).await {
					let title = pst.title;
          let p_title_slug = slug::slugify(&bleach_slug(&title));
				  let link = format!("https://nooki.me/post/{p_title_slug}");
					(title, link, pst.content)
				} else {
					(post_uri, String::new(), String::new())
				};

				let rec = AtRecord {
					kind: String::from("nooki"),
					title: format!("Upvote: {}", title),
					link,
					content,
					images: vec![],
					timestamp: str_to_timestamp(&entry.createdAt) / 1000,
				};
				at_records.push(rec);
			}
		}
	}

	return (at_records, None);
}

async fn get_post_record(id: &str, did: &str, serv: &str) -> Option<PostValue> {
	match get_record("community.nooki.posts", id, &serv, did).await {
		Ok(raw_data) => match serde_json::from_str::<PostResp>(&raw_data) {
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

fn bleach_slug(s: &str) -> String {
	// FIXME: filter out emoji, punctuation 
	s.replace("-", " ").chars().filter(|c| c.is_ascii() && !c.is_ascii_punctuation()).collect()
}
