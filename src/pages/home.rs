use leptos::html::{Input, Textarea};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_meta::Title;
use leptos_use::{use_textarea_autosize, UseTextareaAutosizeReturn};

use crate::helper::utils::go_to;
use crate::resources::atpage::{fetch_atpage, put_atpage_record, AtPageValue, LinkEntry};
use crate::resources::auth::{get_profile, use_auth};

#[component]
pub fn Home() -> impl IntoView {
	let handle_ref = NodeRef::<Input>::new();

	view! {
		<Title text="ATPage" />
		<div class="min-h-screen w-screen" style="background-color: #1a404f;">
			<div class="flex flex-col items-center justify-center px-4 pt-8 mx-auto max-w-xl">
				<h1 class="text-4xl text-center mt-8" style="color: #d2e823;">
					"atpage.one"
				</h1>
				<h2 class="text-2xl text-center mt-2" style="color: #d2e823;">
					"Links and Footprints on atproto, into one customizable page."
				</h2>
				<p class="p-2" style="color: white;">
					"One link in bio for ATmosphere"
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
							let url = if handle.trim().is_empty() {
								format!("/atpage.one")
							} else {
								format!("/{}", handle)
							};
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
				<div class="flex items-center justify-center gap-4 mt-8 absolute bottom-8">
					<a href="https://bsky.app/profile/atpage.one" target="_blank" title="Bluesky">
						<img class="h-6 w-6 hover:scale-[110%]" src="/bluesky.svg" loading="lazy" />
					</a>
					<a href="/atpage.one" title="ATPage">
						<img class="h-6 w-6 hover:scale-[110%]" src="/favicon.svg" loading="lazy" />
					</a>
					<a href="https://github.com/danloh/atpage" target="_blank" title="Source Code">
						<img class="h-6 w-6 hover:scale-[110%]" src="/github.svg" loading="lazy" />
					</a>
				</div>
			</div>
		</div>
	}
}

/// (name, category, domain)
pub const AT_SERVICES: [(&str, &str, &str); 7] = [
	("frontpage", "Link aggregator", "https://frontpage.fyi"),
	("pinksea", "Oekaki BBS", "https://pinksea.art"),
	("ruthub", "Tracker", "https://ruthub.com"),
	("whitewind", "Blog", "https://whtwnd.com"),
	("smokesignal", "Events", "https://smokesignal.events"),
	("grain", "Photo sharing", "https://grain.social"),
	("recipe", "Recipe sharing", "https://recipe.exchange"),
];

/// endpoint `/setup`
#[component]
pub fn SetupPage() -> impl IntoView {
	let auth = use_auth("/setup");
	if auth.get().is_null() {
		go_to("/login?to=/setup");
	}

	let profile = LocalResource::new(move || get_profile(auth.get().handle));

	view! {
		<Title text="Setup atpage" />
		<div class="flex flex-col items-center justify-center p-2 mx-auto max-w-2xl">
			<Suspense fallback=move || "Loading" >
				{move || match profile.get() {
					Some(p) => {
						match p {
							Ok(profile) => {
								view! {
									<div class="flex flex-col gap-2 w-full mb-4 break-all">
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
	let atpage = LocalResource::new(move || fetch_atpage(did.clone()));

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
	let link_form: RwSignal<LinkEntry> = RwSignal::new(LinkEntry::default());

	let UseTextareaAutosizeReturn { content: _, set_content: set_desc, trigger_resize: _ } =
		use_textarea_autosize(desc_ref);

	let UseTextareaAutosizeReturn { content: _, set_content: set_style, trigger_resize: _ } =
		use_textarea_autosize(style_ref);

	let UseTextareaAutosizeReturn { content: _, set_content: set_script, trigger_resize: _ } =
		use_textarea_autosize(script_ref);

	view! {
		<div class="flex flex-col gap-2 w-full h-full mb-4">
		  <b class="text-start hidden">"Title"</b>
			<textarea
				node_ref=title_ref
				name="title"
				id="title"
				class="textarea font-bold w-full hidden"
				prop:value={val.title.unwrap_or_default()}
				placeholder="Title"
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
			></textarea>
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
							  {format!("{}: {} - ", t.0, t.1)}<a class="link link-hover" href={t.2} target="_blank">{t.2}</a>
							</label>
						</div>
					}
				})
				.collect_view()
			}
			<a
			  class="link link-hover text-primary text-xs"
			  href="https://github.com/danloh/atpage/issues/new?template=new_service.md"
				target="_blank"
			>
				"Want more services available? submit please."
		  </a>
			<div class="flex flex-wrap items-center justify-between gap-2">
			  <b class="text-start">"Links on my atpage"</b>
				<button
					class="btn btn-xs btn-ghost rounded-sm p-1 m-1 text-success"
					title="Add Link"
					on:click=move |_| {
						show_add.set(!show_add.get());
						link_form.set(Default::default());
					}
				>
					{move || if show_add.get() {"Cancel"} else {"Add Link"}}
				</button>
			</div>
			<Show when=move || { show_add.get() }>
				<LinkForm links link=link_form.get() show=show_add />
			</Show>
			<div class="flex flex-col items-center justify-center gap-2 w-full">
				{move || links
					.get()
					.iter()
					.map(|link| {
						view! { <LinkBox links link=link.clone() link_form show_add /> }
					})
					.collect_view()
				}
			</div>
			<div class="flex flex-wrap items-center justify-between gap-2">
			  <b class="text-start">"Customize styles of my atpage(Optional)"</b>
				<a
					class="link link-hover text-xs text-primary"
					href="https://github.com/danloh/atpage/tree/main/shared"
					target="_blank"
				>
					"Shared"
				</a>
			</div>
			<div class="flex flex-wrap items-center justify-start gap-2">
			  <span class="text-sm text-primary">
					"Customize or use default style(no input)"
				</span>
				<button
					class="btn btn-xs btn-ghost text-xs text-success"
					on:click=move |_| _ = window().navigator().clipboard().write_text(DEFAULT_STYLE)
				>
					"Copy default style"
				</button>
			</div>
			<textarea
				prop:value={val.style.unwrap_or_default()}
				node_ref=style_ref
				name="style"
				id="style"
				class="textarea h-full w-full code-box"
				placeholder="Customize the style/css of my atpage"
				on:input=move |evt| set_style.set(event_target_value(&evt))
			></textarea>
			<div class="flex flex-wrap items-center justify-between gap-2">
			  <b class="text-start">"Inject script into my atpage(Optional)"</b>
				<a
					class="link link-hover text-xs text-primary"
					href="https://github.com/danloh/atpage/tree/main/shared"
					target="_blank"
				>
					"Shared"
				</a>
			</div>
			<textarea
				prop:value={val.script.unwrap_or_default()}
				node_ref=script_ref
				name="script"
				id="script"
				class="textarea h-full w-full code-box"
				placeholder="Inject JavaScript"
				on:input=move |evt| set_script.set(event_target_value(&evt))
			></textarea>
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
	let category_ref = NodeRef::<Input>::new();
	let icon_ref = NodeRef::<Input>::new();
	let style_ref = NodeRef::<Textarea>::new();

	let disable_btn = RwSignal::new(false);
	let more_option = RwSignal::new(false);

	view! {
		<div class="card w-full h-full mb-4">
			<input
				node_ref=name_ref
				type="text"
				name="name"
				id="name"
				title="Name"
				class="input w-full"
				value={link.name}
				placeholder="Name"
				required
			/>
			<input
				node_ref=url_ref
				name="url"
				id="url"
				title="Link Address"
				class="input w-full"
				placeholder="Link Address"
				prop:value={link.url}
				required
			/>
			<input
				node_ref=icon_ref
				name="icon"
				id="icon"
				title="icon link"
				class="input w-full"
				placeholder="icon link for the link(Optional)"
				prop:value={link.icon.clone().unwrap_or_default()}
			/>
			<input
				node_ref=desc_ref
				name="description"
				id="description"
				title="description"
				class="input w-full"
				placeholder="description(Optional)"
				prop:value={link.description.unwrap_or_default()}
			/>
			<Show when=move || { more_option.get() }>
			  <div class="flex flex-col mt-1">
				  <div class="flex flex-wrap items-center justify-start gap-2">
						<span class="text-sm text-primary my-1">"Style the Link"</span>
						<button
							class="btn btn-xs btn-ghost text-xs text-success"
							on:click=move |_| _ = window().navigator().clipboard().write_text(LINK_STYLE)
						>
							"Copy the snippet"
						</button>
					</div>
					<code class="text-xs my-1">{LINK_STYLE}</code>
					<textarea
						node_ref=style_ref
						name="style"
						id="style"
						title="style the link mt-1"
						class="textarea w-full code-box"
						placeholder="style the link as banner, card..."
						prop:value={link.style.clone().unwrap_or_default()}
					/>
					<input
						node_ref=category_ref
						name="category"
						id="category"
						title="category"
						class="input w-full my-1"
						placeholder="category(Optional)"
						prop:value={link.category.clone().unwrap_or_default()}
					/>
				</div>
			</Show>
			<div class="flex flex-wrap items-start justify-between gap-2 mt-2">
				<button
					class="btn btn-xs btn-ghost text-xs text-success"
					on:click=move |_| { more_option.set(!more_option.get()) }
				>
					{move || if more_option.get() {"Less Options"} else {"More options"}}
				</button>
				<button
					class="btn btn-sm text-success"
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
									category: category_ref.get().map(|d| d.value()),
									style: style_ref.get().map(|s| s.value()),
									icon: icon_ref.get().map(|s| s.value()),
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
	link_form: RwSignal<LinkEntry>,
	show_add: RwSignal<bool>,
) -> impl IntoView {
	let url = link.url.clone();
	let link0 = link.clone();
	view! {
		<div class="w-full flex flex-col items-center justify-center p-2 bg-base-300 rounded">
			<div class="w-full flex flex-wrap items-center justify-between gap-2">
				<div class="flex items-center justify-start">
				  <span class="text-start text-sm">{link.name}</span>
					<button
						class="btn btn-xs btn-ghost text-success"
						title="Edit Link"
						on:click=move |event| {
							event.prevent_default();
							link_form.set(link0.clone());
							show_add.set(true);
						}
					>
						"⋮"
					</button>
				</div>
				<a href={link.url} target="_blank" class="link link-hover text-xs">
				  {link.url.clone()}
				</a>
				<button
					class="btn btn-xs btn-ghost text-warning"
					title="Remove Link"
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
			<div class="w-full flex flex-wrap gap-2 text-xs opacity-75">
			  <span class="text-primary">{link.category}</span><span>{link.description}</span>
			</div>
			<div class="w-full text-xs opacity-75">{link.icon}</div>
			<code class="w-full text-xs opacity-75 mt-2">{link.style}</code>
		</div>
	}
}


pub const DEFAULT_STYLE: &str = "
	body {
		background-color: #fff8e3; 
	}
	.at-page {
	  max-width: 520px;
	}
	.at-card {    
		padding: 10px;
		margin: 5px auto;
		background: #dee9de;
		word-break: break-word;
		border-radius: 8px;
		box-sizing: border-box;
	}
	.at-hdl {
		color: rgb(25, 137, 254);
	}
	.at-kind {
		color: green;
	}
";

pub const LINK_STYLE: &str = 
  "padding: 10px 18px; background-color: blue; color: white; border-radius: 5%;";
