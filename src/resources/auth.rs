use codee::string::JsonSerdeCodec;
use leptos::{prelude::*, task::spawn_local};
use leptos_use::storage::use_local_storage;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::helper::url::get_url;

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct AuthToken {
	pub did: String,
	pub handle: String,
	pub accessJwt: String,
	pub refreshJwt: String,
}

#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthData {
	pub host: String,
	pub did: String,
	pub handle: String,
	pub access: String,
	pub refresh: String,
	pub service: String,
}

impl AuthData {
	pub fn is_null(&self) -> bool {
		self.did.is_empty()
			|| self.handle.is_empty()
			|| self.access.is_empty()
			|| self.refresh.is_empty()
			|| self.host.is_empty()
			|| self.service.is_empty()
	}

	pub async fn new(host: &str, resp: &str) -> Self {
		let json: AuthToken = serde_json::from_str(resp).unwrap();
		let serv = resolve_did(&json.did).await;
		let data = AuthData {
			host: host.to_string(),
			did: json.did,
			handle: json.handle,
			access: json.accessJwt,
			refresh: json.refreshJwt,
			service: serv.service,
		};

		data
	}
}

pub fn use_auth(to: &str) -> Signal<AuthData> {
	let to = to.to_owned();
	spawn_local(async move {
		refresh(&to).await;
	});

	let (auth, _, _) = use_local_storage::<AuthData, JsonSerdeCodec>("auth");

	auth
}

pub async fn login(handle: String, pass: String, host: String) -> AuthData {
	let url = format!("https://{host}/xrpc/com.atproto.server.createSession");

	let mut auth_map = HashMap::new();
	auth_map.insert("identifier", &handle);
	auth_map.insert("password", &pass);

	let client = reqwest::Client::new();
	let res = client.post(url).json(&auth_map).send().await.unwrap().text().await.unwrap();

	// gloo::console::log!("login res: {}", &res);
	let (_, set_auth, _) = use_local_storage::<AuthData, JsonSerdeCodec>("auth");
	let data = AuthData::new(&host, &res).await;
	set_auth.set(data.clone());

	return data;
}

pub async fn refresh(to: &str) {
	let (auth, set_auth, del_auth) = use_local_storage::<AuthData, JsonSerdeCodec>("auth");
	if auth.get().is_null() {
		del_auth();
		window().open_with_url_and_target(&format!("/login?to={to}"), "_self").unwrap();
	}

	let session = get_session(auth.get().access).await;
	if session == "err" {
		let res = refresh_token(auth.get().refresh).await;
		if res == "err" {
			del_auth();
			window().open_with_url_and_target(&format!("/login?to={to}"), "_self").unwrap();
		}
		// gloo::console::log!("{}", &res);
		let data = AuthData::new(&auth.get().host, &res).await;
		set_auth.set(data);
	}
}

pub async fn get_session(access_token: String) -> String {
	let url = get_url("session_get");
	if url.trim().is_empty() {
		return "err".to_string();
	}

	let client = reqwest::Client::new();
	let resp =
		client.get(url).header("Authorization", format!("Bearer {access_token}")).send().await;

	if let Ok(res) = resp {
		match res.error_for_status_ref() {
			Ok(_) => {
				return res.text().await.unwrap_or_else(|_| "err".to_string());
			}
			Err(_e) => {
				return "err".to_string();
			}
		}
	} else {
		return "err".to_string();
	}
}

pub async fn refresh_token(token: String) -> String {
	let url = get_url("session_refresh");
	if url.trim().is_empty() {
		return "err".to_string();
	}

	let client = reqwest::Client::new();
	let resp = client.post(url).header("Authorization", format!("Bearer {token}")).send().await;

	if let Ok(res) = resp {
		match res.error_for_status_ref() {
			Ok(_) => {
				return res.text().await.unwrap_or_else(|_| "err".to_string());
			}
			Err(_e) => {
				return "err".to_string();
			}
		}
	} else {
		return "err".to_string();
	}
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct ResolveRes {
	// pub id: String,
	// pub alsoKnownAs: Vec<String>,
	pub service: Vec<ResolveService>,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct ResolveService {
	// pub id: String,
	pub serviceEndpoint: String,
}

#[derive(Clone, Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct ResolveDid {
	// pub id: String,
	// pub handle: String,
	pub service: String,
}

pub async fn resolve_did(did: &str) -> ResolveDid {
	let fetch_uri = format!("https://plc.directory/{}", did);

	let client = reqwest::Client::new();
	let res = client.get(fetch_uri).send().await.unwrap().text().await.unwrap();

	// gloo::console::log!("req res: ", res.clone());

	let json: ResolveRes = serde_json::from_str(&res).unwrap();

	let final_res = ResolveDid {
		// id: json.id.clone(),
		// handle: json.alsoKnownAs.first().map(|h| h.to_string()).unwrap_or_else(|| json.id),
		service: json.service.first().map(|s| s.serviceEndpoint.clone()).unwrap_or_default(),
	};

	// gloo::console::log!("final res: ", format!("{:?}", final_res));

	final_res
}

/// the typed return of `getProfile` as per atproto api
#[derive(Deserialize, Clone, Default)]
#[allow(non_snake_case)]
pub struct ProfileRes {
  pub did: String,
	pub handle: String,
	pub displayName: Option<String>,
	pub avatar: Option<String>,
	pub description: Option<String>,
}

/// fetch did or handle's profile
pub async fn get_profile(did: String) -> Result<ProfileRes, String> {
	let fetch_uri = format!("https://api.bsky.app/xrpc/app.bsky.actor.getProfile?actor={}", did);

	let client = reqwest::Client::new();
	let res = client
		.get(fetch_uri)
		.send()
		.await
		.map_err(|e| format!("get_profile request err: {:?}", e))?
		.text()
		.await
		.map_err(|e| format!("get_profile err: {:?}", e))?;

	// gloo::console::log!("profile res: ", res.clone());

	serde_json::from_str::<ProfileRes>(&res).map_err(|e| format!("Deserializing err: {:?}", e))
}
