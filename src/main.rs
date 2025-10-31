mod resources;
mod pages;
mod helper;

use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use leptos_router::{components::{Route, Router, Routes}, path};
use pages::{atpage::AtPage, auth::LogIn, home::{Home, SetupPage}};

#[component]
fn App() -> impl IntoView {
	provide_meta_context();

	view! {
		<Router>
			<Routes fallback=|| "Not found">
			  <Route path=path!("/") view=move || view! { <Home /> } />
				<Route path=path!("/login") view=move || view! { <LogIn /> } />
				<Route path=path!("/setup") view=move || view! { <SetupPage /> } />
				<Route path=path!("/:handle") view=move || view! { <AtPage /> } />
			  // <Route path=path!("/t/:did") view=move || view! { <pages::tpage::TestAtPage /> } />
			</Routes>
			<div id="at-toast"></div>
		</Router>
	}
}

fn main() {
  mount_to_body(App);
}
