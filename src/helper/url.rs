use leptos::prelude::Get;
use leptos_use::storage::use_local_storage;
use codee::string::JsonSerdeCodec;
use crate::resources::auth::AuthData;

pub struct BaseUrl {
	pub session_refresh: String,
	pub session_get: String,
	
	// pub record_create: String,
	// pub profile_get: String,
	// pub session_create: String,
	// pub resolve_handle: String,
	// pub thread_get: String,
	// pub describe: String,
	// pub record_list: String,
	// pub record_delete: String,
	// pub timeline_get: String,
	// pub timeline_author: String,
	// pub upload_blob: String,
	// pub update_handle: String,
	// pub account_create: String,
	// pub notify_count: String,
	// pub notify_list: String,
	// pub notify_update: String,
	// pub repo_update: String,
	// pub like: String,
	// pub repost: String,
	// pub follow: String,
	// pub follows: String,
	// pub followers: String,
}

impl BaseUrl {
  fn default() -> Self {
    BaseUrl {
      session_refresh: "com.atproto.server.refreshSession".to_string(),
      session_get: "com.atproto.server.getSession".to_string(),

      // record_create: "com.atproto.repo.createRecord".to_string(),
		  // profile_get: "app.bsky.actor.getProfile".to_string(),
      // session_create: "com.atproto.server.createSession".to_string(),
      // resolve_handle: "com.atproto.identity.resolveHandle".to_string(),
      // thread_get: "app.bsky.feed.getPostThread".to_string(),
      // record_delete: "com.atproto.repo.deleteRecord".to_string(),
      // describe: "com.atproto.repo.describeRepo".to_string(),
      // record_list: "com.atproto.repo.listRecords".to_string(),
      // timeline_get: "app.bsky.feed.getTimeline".to_string(),
      // timeline_author: "app.bsky.feed.getAuthorFeed".to_string(),
      // like: "app.bsky.feed.like".to_string(),
      // repost: "app.bsky.feed.repost".to_string(),
      // follow: "app.bsky.graph.follow".to_string(),
      // follows: "app.bsky.graph.getFollows".to_string(),
      // followers: "app.bsky.graph.getFollowers".to_string(),
      // upload_blob: "com.atproto.repo.uploadBlob".to_string(),
      // account_create: "com.atproto.server.createAccount".to_string(),
      // update_handle: "com.atproto.identity.updateHandle".to_string(),
      // notify_count: "app.bsky.notification.getUnreadCount".to_string(),
      // notify_list: "app.bsky.notification.listNotifications".to_string(),
      // notify_update: "app.bsky.notification.updateSeen".to_string(),
      // repo_update: "com.atproto.sync.updateRepo".to_string(),
    }
  }
}

pub fn get_url(ty: &str) -> String {
  let (auth, _, _) = use_local_storage::<AuthData, JsonSerdeCodec>("auth");
	if auth.get().is_null() {
		return String::new();
	}

	let t = format!("https://{}/xrpc/", auth.get().host);
	let baseurl = BaseUrl::default();
	match ty {
		"session_refresh" => t + &baseurl.session_refresh,
		"session_get" => t + &baseurl.session_get,

		// "record_create" => t + &baseurl.record_create,
		//"profile_get" => t + &baseurl.profile_get,
		// "session_create" => t + &baseurl.session_create,
		// "resolve_handle" => t + &baseurl.resolve_handle,
		// "thread_get" => t + &baseurl.thread_get,
		// "describe" => t + &baseurl.describe,
		// "record_list" => t + &baseurl.record_list,
		// "record_delete" => t + &baseurl.record_delete,
		// "timeline_get" => t + &baseurl.timeline_get,
		// "timeline_author" => t + &baseurl.timeline_get,
		// "upload_blob" => t + &baseurl.upload_blob,
		// "account_create" => t + &baseurl.account_create,
		// "update_handle" => t + &baseurl.update_handle,
		// "notify_list" => t + &baseurl.notify_list,
		// "notify_count" => t + &baseurl.notify_count,
		// "notify_update" => t + &baseurl.notify_update,
		// "repo_update" => t + &baseurl.repo_update,
		// "like" => t + &baseurl.like,
		// "repost" => t + &baseurl.repost,
		// "follow" => t + &baseurl.follow,
		// "follows" => t + &baseurl.follows,
		// "followers" => t + &baseurl.followers,
		_ => String::new(),
	}
}
