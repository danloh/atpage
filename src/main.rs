mod resources;
mod pages;
mod helper;

use std::time::Duration;
use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use leptos_router::{components::{Route, Router, Routes}, path};
use pages::{atpage::AtPage, auth::LogIn, home::{Home, SetupPage}};
// use pages::test_page::TestAtPage;

pub fn toast(text: String) {
  use_context::<Callback<String, ()>>()
	  .expect("Cannot send toast from here")
		.run(text);
}

#[component]
fn App() -> impl IntoView {
	// provide context
	provide_meta_context();

	// gloo::console::log!("starting app: 0");

	// for toast message 
	let show_toast = RwSignal::new(false);
  let toast_text = RwSignal::new(String::new());
  provide_context(Callback::new(move |text: String| {
    toast_text.set(text);
    set_timeout(move || {
			toast_text.set(String::new());
			show_toast.set(false);
		}, Duration::from_secs(10));
    show_toast.set(true);
  }));

	view! {
		<Router>
			<Routes fallback=|| "Not found">
			  <Route path=path!("/") view=move || view! { <Home /> } />
				<Route path=path!("/login") view=move || view! { <LogIn /> } />
				<Route path=path!("/setup") view=move || view! { <SetupPage /> } />
				<Route path=path!("/:handle") view=move || view! { <AtPage /> } />
				// <Route path=path!("/test/:did") view=move || view! { <TestAtPage /> } />
			</Routes>
			<div id="toast" class:show=show_toast>
				{toast_text}
			</div>
		</Router>
	}
}

fn main() {
  mount_to_body(App);
}
