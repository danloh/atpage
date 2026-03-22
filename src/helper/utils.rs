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

/// get icon URL for a service
pub fn get_service_icon(service: &str) -> String {
	match service.to_lowercase().as_str() {
		"frontpage" => "https://frontpage.fyi/frontpage-logo.svg".to_string(),
		"nooki" => "https://icons.duckduckgo.com/ip3/nooki.me.ico".to_string(),
		"pinksea" => "https://icons.duckduckgo.com/ip3/pinksea.art.ico".to_string(),
		"ruthub" => "https://ruthub.com/favicon.svg".to_string(),
		"smokesignal" => "https://icons.duckduckgo.com/ip3/smokesignal.events.ico".to_string(),
		// "grain" => "https://icons.duckduckgo.com/ip3/grain.social.ico".to_string(),
		"recipe" => "https://icons.duckduckgo.com/ip3/recipe.exchange.ico".to_string(),
		"leaflet" => "https://icons.duckduckgo.com/ip3/leaflet.pub.ico".to_string(),
		"popfeed" => "https://icons.duckduckgo.com/ip3/popfeed.social.ico".to_string(),
		"whitewind" => "https://icons.duckduckgo.com/ip3/whtwnd.com.ico".to_string(),
		_ => format!("https://ui-avatars.com/api/?name={}&background=random&color=fff", service),
	}
}
