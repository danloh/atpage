use chrono::{DateTime, Utc};
use leptos::prelude::request_animation_frame;
use leptos_router::hooks::use_navigate;
use leptos_router::NavigateOptions;

/// navigate page
pub fn go_to(page: impl AsRef<str>) {
	let navigate = use_navigate();
	let page = page.as_ref().to_string();
	request_animation_frame(move || {
		navigate(&page, NavigateOptions::default());
	});
}

/// timestamp(sec) to formated str
pub fn ts_to_dt(ts: i64) -> String {
	let dt = DateTime::from_timestamp(ts, 0).unwrap_or_else(|| Utc::now());
	format!("{}", dt.format("%d %b %Y %a %H:%M"))
}

/// formatted datetime str to the number of non-leap-milliseconds
pub fn str_to_timestamp(dt: &str) -> i64 {
	let parsed = dt
		.parse::<DateTime<Utc>>()
		.map(|d| d.timestamp_millis())
		.unwrap_or_else(|_| Utc::now().timestamp_millis());

	parsed
}

/// get URL's domain name
pub fn get_host(uri: &str) -> String {
	let new_uri = uri.replace("http://", "").replace("https://", "").replace("www.", "");
	let parts: Vec<&str> = new_uri.split("/").collect();

	parts.first().map(|s| s.to_string()).unwrap_or_else(|| uri.to_string())
}

/// get ico of a webpage via duckduckgo
pub fn get_ico(uri: &str) -> String {
	let hostname = get_host(uri);
	format!("https://icons.duckduckgo.com/ip3/{hostname}.ico")
}
