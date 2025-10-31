//! Federated link aggregator
//! lexicons: 
//! https://github.com/likeandscribe/frontpage/blob/main/lexicons/fyi/unravel/frontpage/post.json

use serde::{Deserialize, Serialize};

use crate::{
	helper::utils::str_to_timestamp,
	resources::{
		atpage::AtRecord, auth::resolve_did, record::{get_record, list_record, uri_parts}
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
	#[serde(rename = "$type")] // fyi.unravel.frontpage.comment
	pub typ: String, 
	pub content: String, 
	pub post: PostSub,
	pub createdAt: String,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct VotesResp {
	pub records: Vec<VoteResp>,
	pub cursor: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct VoteResp {
	pub uri: String,
	pub cid: String,
	pub value: VoteValue,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct VoteValue {
	#[serde(rename = "$type")] // fyi.unravel.frontpage.vote
	pub typ: String, 
	pub subject: PostSub,
	pub createdAt: String,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PostSub {
	pub uri: String,
	pub cid: String,
}

// TODO, leverage cursor for pagination
pub async fn get_frontpage_records(
	did: &str,
	serv: &str,
	cur: Option<String>,
) -> (Vec<AtRecord>, Option<String>) {
	let mut at_records: Vec<AtRecord> = Vec::new();
	// get posts 
	if let Ok(pst) = list_record("fyi.unravel.frontpage.post", serv, did, cur.clone()).await {
		if let Ok(res) = serde_json::from_str::<FrontpageListResp>(&pst) {
			for entry in res.records {
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
				at_records.push(rec);
			}
		}
	}

	// get comments 
	if let Ok(pst) = list_record("fyi.unravel.frontpage.comment", serv, did, cur.clone()).await {
		if let Ok(res) = serde_json::from_str::<CommentsResp>(&pst) {
			for entry in res.records {
				let value = entry.value;
				let p_uri = value.post.uri;
				let (p_did, _p_col, p_rkey) = uri_parts(&p_uri);
				let link = format!("https://frontpage.fyi/post/{p_did}/{p_rkey}");
				let p_serv = if did == p_did {
					serv.to_string()
				} else { 
					resolve_did(&p_did).await.service
				};
				let title = if let Some(pst) = get_post_record(&p_rkey, &p_did, &p_serv).await {
					pst.title
				} else {
					String::new()
				};

				let rec = AtRecord {
					kind: String::from("frontpage"),
					title: format!("Comment on: {}", title),
					link,
					content: value.content,
					images: Vec::new(),
					timestamp: str_to_timestamp(&value.createdAt) / 1000,
				};
				at_records.push(rec);
			}
		}
	}

	// get votes 
	if let Ok(pst) = list_record("fyi.unravel.frontpage.vote", serv, did, cur.clone()).await {
		if let Ok(res) = serde_json::from_str::<VotesResp>(&pst) {
			for entry in res.records {
				let value = entry.value;
				let s_uri = value.subject.uri;
				let (s_did, s_col, s_rkey) = uri_parts(&s_uri);
				let s_serv = if did == s_did {
					serv.to_string()
				} else { 
					resolve_did(&s_did).await.service
				};

				let (ty, title, link, content) = if s_col == "fyi.unravel.frontpage.post" {
					let link = format!("https://frontpage.fyi/post/{s_did}/{s_rkey}");
					if let Some(pst) = get_post_record(&s_rkey, &s_did, &s_serv).await {
						(String::from("Post"), pst.title, link, String::new())
					} else {
						(String::from("Post"), String::from("link to post"), link, String::new())
					}
				} else if s_col == "fyi.unravel.frontpage.comment" {
					if let Some(cmt) = get_comment_record(&s_rkey, &s_did, &s_serv).await {
						let p_uri = cmt.post.uri;
						let (p_did, _p_col, p_rkey) = uri_parts(&p_uri);
						let link = format!("https://frontpage.fyi/post/{p_did}/{p_rkey}");
						let p_serv = if did == p_did {
							serv.to_string()
						} else { 
							resolve_did(&s_did).await.service
						};
						if let Some(pst) = get_post_record(&p_rkey, &p_did, &p_serv).await {
							(String::from("Comment"), pst.title, link, cmt.content)
						} else {
							(String::from("Comment"), String::from("link to post"), link, cmt.content)
						}
					} else {
						continue;
					}
				} else {
					continue;
				};

				let rec = AtRecord {
					kind: String::from("frontpage"),
					title: format!("Vote on {}: {}", ty, title),
					link,
					content,
					images: Vec::new(),
					timestamp: str_to_timestamp(&value.createdAt) / 1000,
				};
				at_records.push(rec);
			}
		}
	}

	return (at_records, None);
}

async fn get_post_record(id: &str, did: &str, serv: &str) -> Option<FrontpageValue> {
	match get_record("fyi.unravel.frontpage.post", id, &serv, did).await {
		Ok(raw_data) => match serde_json::from_str::<FrontpageResp>(&raw_data) {
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

async fn get_comment_record(id: &str, did: &str, serv: &str) -> Option<CommentValue> {
	match get_record("fyi.unravel.frontpage.comment", id, &serv, did).await {
		Ok(raw_data) => match serde_json::from_str::<CommentResp>(&raw_data) {
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
