use serde::{Deserialize, Serialize};

/// for put record response
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PutResp {
	pub uri: String,
	pub cid: String,
}

// FIXME, how to handle panic better?
/// split at: URI, return (did, collection, rkey)
pub fn uri_parts(uri: &str) -> (String, String, String) {
	// uri: at://<DID>/<COLLECTION>/<RKEY>
	let lst: Vec<&str> = uri.split("/").collect();
	if lst.len() < 5 {
		return (String::new(), String::new(), String::new());
	}
	(lst[2].to_string(), lst[3].to_string(), lst[4].to_string())
}

pub async fn get_record(col: &str, rkey: &str, serv: &str, did: &str) -> Result<String, String> {
	let fetch_uri = format!(
		"{}/xrpc/com.atproto.repo.getRecord?repo={}&collection={}&rkey={}",
		serv, did, col, rkey
	);

	let client = reqwest::Client::new();
	let res = client
		.get(fetch_uri)
		.send()
		.await
		.map_err(|e| format!("get_record request err: {:?}", e))?
		.text()
		.await
		.map_err(|e| format!("get_record err: {:?}", e))?;

	Ok(res)
}

pub async fn list_record(
	col: &str,
	serv: &str,
	did: &str,
	cursor: Option<String>,
) -> Result<String, String> {
	let fetch_uri = if let Some(cur) = cursor {
		format!(
			"{}/xrpc/com.atproto.repo.listRecords?repo={}&collection={}&cursor={}",
			serv, did, col, cur
		)
	} else {
		format!("{}/xrpc/com.atproto.repo.listRecords?repo={}&collection={}", serv, did, col)
	};

	let client = reqwest::Client::new();
	let res = client
		.get(fetch_uri)
		.send()
		.await
		.map_err(|e| format!("list_record request err: {:?}", e))?
		.text()
		.await
		.map_err(|e| format!("list_record err: {:?}", e))?;

	Ok(res)
}
