use leptos::prelude::*;
use leptos::html::{Input, Textarea};
use leptos::task::spawn_local;
use leptos_meta::Title;
use leptos_use::{use_textarea_autosize, UseTextareaAutosizeReturn};
use crate::helper::utils::go_to;
use crate::resources::atpage::{fetch_atpage, put_atpage_record, LinkEntry, AtPageValue};
use crate::resources::auth::{get_profile, use_auth};

#[component]
pub fn Home() -> impl IntoView {
	let handle_ref = NodeRef::<Input>::new();

	view! {
		<Title text="ATPage" />
		<div class="min-h-screen w-screen" style="background-color: #1a404f;">
			<div class="flex flex-col items-center justify-center px-4 pt-8 mx-auto max-w-xl">
				<h1 class="text-4xl text-center mt-8" style="color: #d2e823;">
					"atpage: all footprints on atproto, into one customizable page."
				</h1>
				<p class="p-2" style="color: white;">
					"One link in bio to help share everything you create on atproto"
				</p>
				<div class="flex items-center justify-center gap-2 w-full mt-4">
					<input 
						node_ref=handle_ref 
						class="input input-bordered " 
						placeholder="/handle" 
					/>
					<button 
					  class="w-12 btn rounded"
						on:click=move |_| {
							let handle = handle_ref.get().unwrap().value();
							if handle.trim().is_empty() {
								return;
							}
							let url = format!("/{}", handle);
							window().open_with_url_and_target(&url, "_self").unwrap();
						}
					>
						"View"
					</button>
					<button 
					  class="w-16 btn rounded"
						style="background-color: #d2e823;"
						on:click=move |_| {
							window().open_with_url_and_target("/setup", "_self").unwrap();
						}
					>
						"Setup"
					</button>
				</div>
				<div class="flex items-center justify-center gap-2 mt-4">
					<a 
						class="text-success text-center link link-hover" 
						href="https://bsky.app/profile/atpage.bsky.social" 
						target="_blank"
					>
						"Bluesky"
					</a>
					<a 
						class="text-success text-center link link-hover" 
						href="https://github.com/danloh/atpage" 
						target="_blank"
					>
						"Source"
					</a>
				</div>
			</div>
		</div>
	}
}

/// (name, category, domain)
pub const AT_SERVICES: [(&str, &str, &str); 4] = [
	("frontpage", "Links", "https://frontpage.fyi"),
	("pinksea", "Oekaki", "https://pinksea.art"), 
	("ruthub", "Tracker", "https://ruthub.com"),
	("whitewind", "Blog", "https://whtwnd.com"), 
];

/// endpoint `/setup`
#[component]
pub fn SetupPage() -> impl IntoView {
	let auth = use_auth("/setup");
	if auth.get().is_null() {
		go_to("/login?to=/setup");
	}

	let profile = LocalResource::new(
		move || get_profile(auth.get().handle)
	);

	view! {
		<Title text="Setup atpage" />
		<div class="flex flex-col items-center justify-center p-2 mx-auto max-w-2xl">
			<Suspense fallback=move || "Loading" >
				{move || match profile.get() {
					Some(p) => { 
						match p {
							Ok(profile) => {
								view! { 
									<div class="flex flex-col gap-2 w-full mb-4">
										<div class="flex flex-col items-center justify-center gap-2 mb-4">
											<div class="flex flex-wrap items-center justify-start gap-2">
												<img 
													class="h-4 w-4 rounded" 
													src={profile.avatar.clone().unwrap_or_default()} 
													loading="lazy" 
												/>
												<a 
													href={format!("https://bsky.app/profile/{}", profile.handle.clone())} 
													target="_blank" 
													class="link link-hover"
												>
													{
														profile.displayName.clone().map(|n| {
															if n.trim().is_empty() { profile.handle.clone() } else { n }
														})
														.unwrap_or_else(|| profile.handle.clone())
													}
												</a>
											</div>
											<div>{profile.description.clone().unwrap_or_default()}</div>
										</div>
										<hr class="p-2" />
										<SetupWrap did=profile.did.clone() />
									</div>
								}
								.into_any()
							}
							Err(_e) => "".into_any()
						}
					}
					None => "".into_any()
				}}
			</Suspense>
		</div>
	}
}

#[component]
pub fn SetupWrap(did: String) -> impl IntoView {
	let atpage = LocalResource::new(
		move || fetch_atpage(did.clone())
	);

	view! {
		<div class="flex flex-col items-center justify-center">
			<Suspense fallback=move || "Loading" >
				{move || match atpage.get() {
					Some(val) => { 
						view! { <SetupForm val=val.unwrap_or_default() /> }.into_any()
					}
					None => "".into_any()
				}}
			</Suspense>
		</div>
	}
}

#[component]
pub fn SetupForm(val: AtPageValue) -> impl IntoView {
	let title_ref = NodeRef::<Textarea>::new();
	let desc_ref = NodeRef::<Textarea>::new();
	let style_ref = NodeRef::<Textarea>::new();
	let script_ref = NodeRef::<Textarea>::new();

	let links: RwSignal<Vec<LinkEntry>> = RwSignal::new(val.links);
  let services: RwSignal<Vec<String>> = RwSignal::new(val.services);
	let disable_btn = RwSignal::new(false);
	let show_add = RwSignal::new(false);

	let UseTextareaAutosizeReturn { content: _, set_content: set_desc, trigger_resize: _ } =
		use_textarea_autosize(desc_ref);
	
	let UseTextareaAutosizeReturn { content: _, set_content: set_style, trigger_resize: _ } =
		use_textarea_autosize(style_ref);

	let UseTextareaAutosizeReturn { content: _, set_content: set_script, trigger_resize: _ } =
		use_textarea_autosize(script_ref);

	view! {
		<div class="flex flex-col gap-2 w-full h-full mb-4">
		  <b class="text-start">"Select ATProto Services"</b>
			{AT_SERVICES
				.into_iter()
				.map(|t| {
					view! {
						<div class="flex flex-wrap items-center justify-start gap-2">
							<input 
								type="checkbox" 
								class="checkbox" 
								value={t.0} 
								checked={move || services.get().contains(&t.0.to_string())}
								on:input=move |evt| {
									let checked = event_target_checked(&evt);
									let s = t.0.to_string();
									if checked {
										services.update(|servs| {
											if !servs.contains(&s) {
												servs.push(s);
											}
										});
									} else {
										services.update(|servs| {
											servs.retain(|serv| serv != &s);
										});
									}
								}
							/>
							<label for={t.0}>
							  {format!("{}: {} - ", t.0, t.1)}<a href={t.2} target="_blank">{t.2}</a>
							</label>
						</div>
					}
				})
				.collect_view()
			}
			<a 
			  class="link link-hover text-success text-xs" 
			  href="https://github.com/danloh/atpage/issues/new?template=new_service.md" 
				target="_blank"
			>
				"Want more services available? submit here."
		  </a>
			<b class="text-start hidden">"Title"</b>
			<textarea
				node_ref=title_ref
				name="title"
				id="title"
				class="textarea font-bold w-full hidden"
				prop:value={val.title.unwrap_or_default()}
				placeholder="Title"
				required
			></textarea>
			<b class="text-start">"Description"</b>
			<textarea
				prop:value={val.description.unwrap_or_default()}
				node_ref=desc_ref
				name="description"
				id="description"
				class="textarea h-full w-full"
				placeholder="Description of my atpage..."
				on:input=move |evt| set_desc.set(event_target_value(&evt))
				required
			></textarea>
			<b class="text-start">"Styles"</b>
			<textarea
				prop:value={val.style.unwrap_or_default()}
				node_ref=style_ref
				name="style"
				id="style"
				class="textarea h-full w-full"
				placeholder="Customize the style/css of my atpage: e.g. body {background-color: blue;} .at-page {color: green;} ..."
				on:input=move |evt| set_style.set(event_target_value(&evt))
				required
			></textarea>
			<b class="text-start">"Script"</b>
			<textarea
				prop:value={val.script.unwrap_or_default()}
				node_ref=script_ref
				name="script"
				id="script"
				class="textarea h-full w-full"
				placeholder="Inject JavaScript"
				on:input=move |evt| set_script.set(event_target_value(&evt))
				required
			></textarea>
			<div class="flex items-center justify-between gap-2">
			  <b class="text-start">"Links"</b>
				<button
					class="btn btn-xs btn-ghost rounded-sm p-1 m-1 text-success"
					title="Add Link"
					on:click=move |_| { show_add.set(!show_add.get()); }
				>
					{move || if show_add.get() {"Cancel"} else {"Add"}}
				</button>
			</div>
			<div class="flex flex-col items-center justify-center gap-2 w-full">
				{move || links
					.get()
					.iter()
					//.filter(predicate)
					.map(|link| {
						view! { <LinkBox links link=link.clone() /> }
					})
					.collect_view()
				}
			</div>
			<Show when=move || { show_add.get() } fallback=|| "" >
				<LinkForm links link=Default::default() show=show_add />
			</Show>
			<button
				class="btn text-success mt-4"
				disabled={disable_btn}
				on:click=move |event| {
					event.prevent_default();
					spawn_local(async move {
						disable_btn.set(true);
						let new_data = AtPageValue {
							services: services.get(),
							links: links.get(),
							title: title_ref.get().map(|t| t.value()),
							description: desc_ref.get().map(|d| d.value()),
							style: style_ref.get().map(|s| s.value()),
							script: script_ref.get().map(|s| s.value()),
							..Default::default()
						};

						// gloo::console::log!("value: ", format!("{:?}", new_data));
						let auth = use_auth("/setup");
						_ = put_atpage_record(auth.get(), &new_data).await;
						
						disable_btn.set(false);
						window()
						  .open_with_url_and_target(&format!("/{}", auth.get().handle), "_self")
							.unwrap();
					});
				}
			>
				"DONE"
			</button>
		</div>
	}
}

#[component]
pub fn LinkForm(
	links: RwSignal<Vec<LinkEntry>>,
	link: LinkEntry,
	show: RwSignal<bool>,
) -> impl IntoView {
	let name_ref = NodeRef::<Input>::new();
	let url_ref = NodeRef::<Input>::new();
	let desc_ref = NodeRef::<Input>::new();
	let style_ref = NodeRef::<Textarea>::new();

	let disable_btn = RwSignal::new(false);

	view! {
		<div class="card w-full h-full">
			<input
				node_ref=name_ref
				type="text"
				name="name"
				id="name"
				class="input w-full"
				value={link.name}
				placeholder="name"
				required
			/>
			<input
				node_ref=url_ref
				name="url"
				id="url"
				class="input w-full"
				placeholder="URL"
				prop:value={link.url}
				required
			/>
			<input
				node_ref=desc_ref
				name="description"
				id="description"
				class="input w-full"
				placeholder="description"
				prop:value={link.description.unwrap_or_default()}
				required
			/>
			<textarea
				node_ref=style_ref
				name="style"
				id="style"
				class="textarea w-full hidden"
				placeholder="style"
				prop:value={link.style.unwrap_or_default()}
				required
			></textarea>
			<div class="flex flex-wrap items-center justify-center gap-2 mt-2">
				<button
					class="btn btn-xs text-warning"
					disabled={disable_btn}
					on:click=move |event| {
						event.prevent_default();
						spawn_local(async move {
							disable_btn.set(true);
							links.update(|lnks| {
								let url = url_ref.get().unwrap().value();
								lnks.retain(|l| l.url != url);
							});
							disable_btn.set(false);
							show.set(false);
						});
					}
				>
					"Remove"
				</button>
				<button
					class="btn btn-xs text-success"
					disabled={disable_btn}
					on:click=move |event| {
						event.prevent_default();
						spawn_local(async move {
							disable_btn.set(true);
							links.update(|lnks| {
								let url = url_ref.get().unwrap().value();
								let new_link = LinkEntry {
									url: url.clone(),
									name: name_ref.get().unwrap().value(),
									description: desc_ref.get().map(|d| d.value()),
									style: style_ref.get().map(|s| s.value()),
									..Default::default()
								};
								lnks.retain(|l| l.url != url);
								lnks.push(new_link);
							});
							disable_btn.set(false);
							show.set(false);
						});
					}
				>
					"Add Link"
				</button>
			</div>
		</div>
	}
}

#[component]
pub fn LinkBox(
	links: RwSignal<Vec<LinkEntry>>,
	link: LinkEntry,
) -> impl IntoView {
	let url = link.url.clone();
	view! {
		<div class="w-full flex flex-col items-center justify-center p-2 bg-base-200 rounded">
			<div class="w-full flex flex-wrap items-center justify-between gap-2">
				<span class="text-start text-success">{link.name}</span>
				<a href={link.url} target="_blank" class="link link-hover text-primary">
				  {link.url.clone()}
				</a>
				<button
					class="btn btn-xs btn-ghost text-warning"
					on:click=move |event| {
						event.prevent_default();
						let url = url.clone();
						spawn_local(async move {
							links.update(|lnks| {
								let url = url.clone();
								lnks.retain(|l| l.url != url);
							});
						});
					}
				>
					"X"
				</button>
			</div>
			<div class="w-full text-sm opacity-75">{link.description}</div>
		</div>
	}
}
