use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::html::Input;
use leptos_meta::Title;
use leptos_router::hooks::use_query_map;
use phosphor_leptos::{Icon, CLOUD, KEY, USER};

use crate::resources::auth::login;

/// endpoint `/login?to=`
#[component]
pub fn LogIn() -> impl IntoView {
  let querys = use_query_map();
	let redirect: RwSignal<String> = RwSignal::new(
    querys.read().get("to").unwrap_or_else(|| "/".to_string())
  ); 

  let handle_ref = NodeRef::<Input>::new();
  let pass_ref = NodeRef::<Input>::new();
  let host_ref = NodeRef::<Input>::new();

  let disable_btn = RwSignal::new(false);
  let on_login = move |handle: String, password: String, host: String| async move {
    let _data = login(handle, password, host.clone()).await;
    window().open_with_url_and_target(&redirect.get(), "_self").unwrap();
  };
  
  view! {
    <Title text="Log in | ATPage" />
    <div class="card mx-auto py-8 px-4 w-full mx-auto max-w-md">
      <h1 class="title text-center text-2xl mb-4">"Welcome to ATPage"</h1>
      <form
        class="flex flex-col gap-4"
        on:submit=move |event| {
          event.prevent_default();
          spawn_local(async move {
            disable_btn.set(true);
            on_login(
              handle_ref.get().unwrap().value(),
              pass_ref.get().unwrap().value(),
              host_ref.get().map(|h| h.value()).unwrap_or_else(|| String::from("bsky.social")),
            )
            .await;
            disable_btn.set(false);
          });
        }
      >
        <label for="host" class="w-full input flex items-center gap-2">
          <Icon icon=CLOUD size="24" /><span class="text-accent">"Service"</span> 
          <input
            node_ref=host_ref
            type="text"
            name="host"
            id="host" 
            placeholder="such as: bsky.social"
            value="bsky.social"
            class="grow" 
          />
        </label>
        <label for="handle" class="w-full input flex items-center gap-2">
          <Icon icon=USER size="24" /><span class="text-accent">"Handle@"</span>
          <input
            node_ref=handle_ref
            type="text"
            name="handle"
            id="handle" 
            placeholder="e.g. my.bsky.social"
            class="grow" 
            required
          />
        </label>
        <label for="pass" class="w-full input flex items-center gap-2">
          <Icon icon=KEY size="24" /><span class="text-accent">"Password"</span>
          <input
            node_ref=pass_ref
            type="password"
            name="pass"
            id="pass" 
            class="grow" 
            placeholder="App Password"
            required
          />
        </label>
        <a 
          class="text-sm text-success" 
          href="https://bsky.app/settings/app-passwords" 
          target="_blank"
        >
          "Go to generate the app password"
        </a>
        <button type="submit" class="btn btn-neutral" disabled={disable_btn}>
          "Log In"
        </button>
      </form>
    </div>
  }
}
