use chrono::{DateTime, Utc};
use leptos::prelude::request_animation_frame;
use leptos_router::hooks::use_navigate;
use leptos_router::NavigateOptions;

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
	format!("{}", dt.format("%d %b %Y, %a"))
}

/// formatted datetime str to the number of non-leap-milliseconds
pub fn str_to_timestamp(dt: &str) -> i64 {
  let parsed = dt
    .parse::<DateTime<Utc>>()
    .map(|d| d.timestamp_millis())
    .unwrap_or_else(|_| Utc::now().timestamp_millis());
  
  parsed
}
