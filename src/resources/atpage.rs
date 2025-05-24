use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::{
	auth::{resolve_did, AuthData},
	record::{get_record, PutResp},
	service::{
		frontpage::get_frontpage_records, pinksea::get_pinksea_records, ruthub::get_ruthub_records, 
		smokesignal::get_smokesignal_records, whitewind::get_wnd_records
	},
};

/// for record links
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AtPageResp {
	pub uri: String,
	pub cid: String,
	pub value: AtPageValue,
}

/// for record links
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct AtPageValue {
	#[serde(rename = "$type")]
	pub kind: String,
	pub links: Vec<LinkEntry>,
	pub services: Vec<String>,
	pub title: Option<String>,
	pub description: Option<String>,
	pub style: Option<String>,
	pub script: Option<String>,
}

/// for record link entry
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct LinkEntry {
	pub url: String,
	pub name: String,
	pub order: i32,
	pub createdAt: i64,
	pub description: Option<String>,
	pub icon: Option<String>,
	pub style: Option<String>,
}

pub async fn fetch_atpage(did: String) -> Option<AtPageValue> {
	let service = resolve_did(&did).await.service;
	get_atpage_record(&did, &service).await
}

pub async fn get_atpage_record(did: &str, service: &str) -> Option<AtPageValue> {
	match get_record("one.atpage.page", "self", service, &did).await {
		Ok(raw_data) => match serde_json::from_str::<AtPageResp>(&raw_data) {
			Ok(data_res) => {
				let value: AtPageValue = data_res.value;
				return Some(value);
			}
			Err(e) => {
				gloo::console::log!("get_atpage_record de error: ", format!("{:?}", e));
				return None;
			}
		},
		Err(e) => {
			gloo::console::log!("get_atpage_record error: ", format!("{:?}", e));
			return None;
		}
	}
}

pub async fn put_atpage_record(usr: AuthData, data: &AtPageValue) -> Result<PutResp, String> {
	let post_uri = format!("{}/xrpc/com.atproto.repo.putRecord", usr.service);

	let post = json!({
		"repo": &usr.did,
		"collection": "one.atpage.page",
		"rkey": "self",
		"record": {
			"$type": "one.atpage.page",
      "links": data.links,
      "services": data.services,
      "title": data.title.clone().unwrap_or_default(),
      "description": data.description.clone().unwrap_or_default(),
      "style": data.style.clone().unwrap_or_default(),
      "script": data.script.clone().unwrap_or_default(),
		},
		"validate": false
	});

	let client = reqwest::Client::new();
	let res = client
		.post(post_uri)
		.json(&post)
		.header("Authorization", format!("Bearer {}", &usr.access))
		.send()
		.await
		.map_err(|e| format!("put_atpage_record request err: {:?}", e))?
		.text()
		.await
		.map_err(|e| format!("put_atpage_record err: {:?}", e))?;

	serde_json::from_str::<PutResp>(&res).map_err(|e| format!("put_atpage_record de err: {:?}", e))
}

// ===================================================================================
// ===========  Service Record =======================================================
// ===================================================================================

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct AtRecord {
	pub kind: String,
	pub title: String,
	pub link: String,
	pub content: String,
	pub images: Vec<String>,
	pub timestamp: i64, // sec
}

#[derive(Clone, Default)]
pub struct RecordsRes {
	pub records: Vec<AtRecord>,
	pub cursor: Option<String>,
}

pub async fn fetch_records((did, services): (String, Vec<String>)) -> RecordsRes {
	let serv = resolve_did(&did).await.service;
	let mut vec = Vec::new();
	for service in services {
		if service == "frontpage" {
			let (mut frontpages, _f_cur) = get_frontpage_records(&did, &serv, None).await;
			vec.append(&mut frontpages);
		}
		if service == "whitewind" {
			let (mut wnds, _w_cur) = get_wnd_records(&did, &serv, None).await;
			vec.append(&mut wnds);
		}
		if service == "pinksea" {
			let (mut pinkseas, _p_cur) = get_pinksea_records(&did, &serv, None).await;
			vec.append(&mut pinkseas);
		}
		if service == "ruthub" {
			let (mut ruts, _r_cur) = get_ruthub_records(&did, &serv, None).await;
			vec.append(&mut ruts);
		}
		if service == "smokesignal" {
			let (mut smokes, _s_cur) = get_smokesignal_records(&did, &serv, None).await;
			vec.append(&mut smokes);
		}
	}

	let final_vec =
		vec.into_iter().sorted_by(|a, b| Ord::cmp(&b.timestamp, &a.timestamp)).collect();

	RecordsRes { records: final_vec, cursor: None }
}
