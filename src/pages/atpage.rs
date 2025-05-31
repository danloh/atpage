use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::hooks::use_params_map;
use phosphor_leptos::{Icon, BUTTERFLY, COPY, FACEBOOK_LOGO, LINKEDIN_LOGO, SHARE_NETWORK, TWITTER_LOGO};

use crate::helper::md::md2html;
use crate::helper::utils::{get_ico, ts_to_dt};
use crate::resources::atpage::{fetch_atpage, fetch_records, AtRecord};
use crate::resources::auth::{get_profile, ProfileRes};

/// endpoint `/:handle`
#[component]
pub fn AtPage() -> impl IntoView {
	let params = use_params_map();
	let handle: String = params.read().get("handle").unwrap_or_default();
	if handle.trim().is_empty() {
		return "No Data".into_any();
	}

	let txt = format!("{} | atpage", &handle);
	let profile = LocalResource::new(move || get_profile(handle.clone()));

	view! {
		<Title text={txt} />
		<Suspense fallback=move || "Loading" >
			{move || match profile.get() {
				Some(p) => {
					match p {
						Ok(profile) => { view! { <AtView profile /> }.into_any() }
						Err(_e) => "".into_any()
					}
				}
				None => "".into_any()
			}}
		</Suspense>
	}
	.into_any()
}

#[component]
pub fn AtView(profile: ProfileRes) -> impl IntoView {
	let p_signal = RwSignal::new(profile);
	let atpage = LocalResource::new(move || fetch_atpage(p_signal.get().did));

	view! {
		<Suspense fallback=move || "Loading">
			{move || match atpage.get() {
				Some(res_) => {
					if let Some(res) = res_ {
						let (is_src, style_css) = match res.style {
							Some(s) if !s.trim().is_empty() => {
								(s.starts_with("https://") && s.ends_with(".css"), s)
							}
							_ => (false, DEFAULT_STYLE.to_string())
						};
						let (is_link, js_script) = match res.script {
							Some(s) if !s.trim().is_empty() => {
								(s.starts_with("https://"), s)
							}
							_ => (false, "".to_string())
						};
						view! {
							{if is_src {
								view! { <Stylesheet href={style_css} /> }.into_any()
							} else {
								view! { <Style>{style_css}</Style> }.into_any()
							}}
							{if is_link {
								view! { <Link rel="preload" href={js_script} as_="script" /> }.into_any()
							} else {
								view! { <Script>{js_script}</Script> }.into_any()
							}}
							<div class="min-h-screen w-screen at-screen">
								<div class="flex flex-col items-center justify-center p-2 mx-auto max-w-2xl at-page">
									<ProfileView profile=p_signal.get() />
									<div class="flex flex-col items-center justify-center gap-2 at-view">
										<div
											class="w-full prose flex items-center justify-center p-2 at-description"
											inner_html={md2html(&res.description.unwrap_or_default()).html}
										/>
										<div class="w-full flex flex-wrap items-center justify-center gap-2 at-links">
											{res
												.links
												.into_iter()
												.map(|link| {
													let url = link.url.clone();
													let name = link.name.clone();
													
													let (link_style, styled) = match link.style {
														Some(s) if !s.trim().is_empty() => (s, true),
														_ => (String::new(), false)
													};
													let link_icon = match link.icon {
														Some(ico) if !ico.trim().is_empty() => ico,
														_ => get_ico(&url)
													};
													// no space allowed in class name
													let cname = name.replace(" ", "");
													let link_card_class = 
														format!("w-full flex gap-2 at-link-card at-link-card-{cname}");
													let link_class = 
														format!("w-full flex gap-2 link link-hover at-link at-link-{cname}");
													let img_class = 
														format!("h-6 w-6 hover:scale-[108%] at-ico at-ico-{}", cname);

													if styled {
														view! {
															<div class={link_card_class} style={link_style}>
																<a
																	class={link_class}
																	href={url.clone()}
																	title={name.clone()}
																>
																	<img
																		class={img_class}
																		src={link_icon}
																		alt={name.clone()}
																		loading="lazy"
																	/>
																	<div class={format!("flex gap-2 at-desc at-desc-{}", cname)}>
																		<span class={format!("at-name at-name-{}", cname)}>
																			{name.clone()}
																		</span>
																		<span class={format!("at-des at-des-{}", cname)}>
																			{link.description.unwrap_or_default()}
																		</span>
																	</div>
																</a>
																<ShareBtn text={url} />
															</div>
														}
														.into_any()
													} else {
														view! {
															<a
																class={format!("link link-hover at-link at-link-{}", cname)}
																href={url}
																title={name.clone()}
															>
																<img
																	class={img_class}
																	src={link_icon}
																	alt={name.clone()}
																	loading="lazy"
																/>
															</a>
														}
														.into_any()
													}
												})
												.collect_view()
											}
										</div>
										<AtBox
											did=p_signal.get().did
											services=res.services.clone()
											profile=p_signal
										/>
									</div>
									<div class="w-full my-4 at-btm">
										<a href="/setup" class="link link-hover at-join">
											{format!(
												"Join {} on ATPage",
												p_signal.get().displayName.clone().map(|n| {
													if n.trim().is_empty() { p_signal.get().handle.clone() } else { n }
												})
												.unwrap_or_else(|| p_signal.get().handle.clone()))
											}
										</a>
									</div>
								</div>
							</div>
						}
						.into_any()
					} else {
						view! {
							<div class="min-h-screen w-screen at-screen">
								<div class="flex flex-col items-center justify-center p-2 mx-auto max-w-2xl">
									<ProfileView profile=p_signal.get() />
									<a class="link link-hover mt-4" href="/setup">"Setup my ATPage"</a>
								</div>
							</div>
						}
						.into_any()
				  }
				}
				None => "error".into_any()
			}}
		</Suspense>
	}
}

#[component]
pub fn AtBox(did: String, services: Vec<String>, profile: RwSignal<ProfileRes>) -> impl IntoView {
	let rec_res = LocalResource::new(move || fetch_records((did.clone(), services.clone())));

	view! {
		<div class="w-full flex flex-col items-center justify-center gap-2 at-box">
			<Suspense fallback=move || "Loading">
				{move || match rec_res.get() {
					Some(res) => {
						res
							.records
							.iter()
						  .map(|rec| view! { <RecCard rec=rec.clone() profile /> })
						  .collect_view()
							.into_any()
					}
					_ => "error".into_any()
				}}
			</Suspense>
		</div>
	}
}

#[component]
pub fn RecCard(rec: AtRecord, profile: RwSignal<ProfileRes>) -> impl IntoView {
	let kd = rec.kind.replace(" ", "");

	view! {
		<div class={format!("w-full p-2 at-card at-card-{}", kd)}>
			<div class={format!("flex items-center justify-start gap-2 at-meta at-meta-{}", kd)}>
				<img
					class={format!("h-4 w-4 rounded at-ava at-ava-{}", kd)}
					src={profile.get().avatar.clone().unwrap_or_default()}
					loading="lazy"
				/>
				<a
					href={format!("https://bsky.app/profile/{}", profile.get().handle.clone())}
					target="_blank"
					class={format!("link link-hover text-xs at-hdl at-hdl-{}", kd)}
				>
					{
						profile.get().displayName.clone().map(|n| {
							if n.trim().is_empty() { profile.get().handle.clone() } else { n }
						})
						.unwrap_or_else(|| profile.get().handle.clone())
					}
			  </a>
				<span class={format!("text-xs at-date at-date-{}", kd)}>
					{ts_to_dt(rec.timestamp)}
				</span>
				<a
					href={rec.link.clone()}
					class={format!("link link-hover text-xs at-kind at-kind-{}", kd)}
					target="_blank"
				>
					{format!("@{}", kd)}
				</a>
				<ShareBtn text={rec.link.clone()} />
			</div>
			<div class={format!("flex items-center justify-start gap-2 at-title at-title-{}", kd)}>
				<a
					href={rec.link}
					target="_blank"
					class={format!("font-bold text-xl link link-hover at-title-link at-title-link-{}", kd)}
				>
					{rec.title}
				</a>
			</div>
			<div class={format!("w-full flex flex-wrap items-center justify-center gap-2 at-images at-images-{}", kd)}>
				{
					rec.images.into_iter().map(|img| view! {
						<img
							class={format!("max-w-full p-2 at-image at-image-{}", kd)}
							src={img}
							loading="lazy"
						/>
					})
					.collect_view()
				}
			</div>
			<div
				class={format!("flex-1 w-full prose no-scrollbar at-ctn at-ctn-{}", kd)}
				style="max-height: 240px; overflow: auto"
				inner_html={md2html(&rec.content).html}
			></div>
		</div>
	}
}

#[component]
pub fn ProfileView(profile: ProfileRes) -> impl IntoView {
	view! {
		<div class="w-full flex flex-wrap items-center justify-center gap-2 at-profile">
			<img
				class="h-8 w-8 rounded-full at-avatar"
				src={profile.avatar.clone().unwrap_or_default()}
				loading="lazy"
			/>
			<a
				href={format!("https://bsky.app/profile/{}", profile.handle.clone())}
				target="_blank"
				class="link link-hover text-2xl at-handle"
			>
				{
					profile.displayName.clone().map(|n| {
						if n.trim().is_empty() { profile.handle.clone() } else { n }
					})
					.unwrap_or_else(|| profile.handle.clone())
				}
			</a>
			<ShareBtn text={format!("https://atpage.one/{}", profile.handle.clone())} />
		</div>
	}
}

#[component]
pub fn ShareBtn(text: String) -> impl IntoView {
	let txt = RwSignal::new(text);

	view! {
		<details class="dropdown dropdown-end at-share">
		  <summary class="btn btn-xs btn-ghost">
				<Icon icon=SHARE_NETWORK size="18" />
			</summary>
			<ul class="menu dropdown-content p-2 bg-base-300 rounded-sm z-50 w-32">
				<div class="flex flex-col h-full overflow-y-auto no-scrollbar">
					<div class="flex flex-col items-start justify-center gap-2">
					  <button
						  class="btn btn-xs btn-ghost rounded-sm p-1 mx-1 flex" 
							style="color: #AE2983;"
	            on:click=move |_| {
								let clipboard = window().navigator().clipboard();
								let _ = clipboard.write_text(&txt.get());
							}
						>
							<Icon icon=COPY size="18px"/> " Copy"
						</button>
						<button
							class="btn btn-xs btn-ghost rounded-sm p-1 mx-1 flex" 
							style="color: #AE2983;"
							on:click=move |_| { 
								let share_url = format!("https://bsky.app/intent/compose?text={}", txt.get());
								_ = window().open_with_url(&share_url);
							}
						>
							<Icon icon=BUTTERFLY size="18px"/> " Bluesky"
						</button>
						<button
							class="btn btn-xs btn-ghost rounded-sm p-1 mx-1 flex" 
							style="color: #AE2983;"
							on:click=move |_| { 
								let share_url = format!(
									"https://www.linkedin.com/sharing/share-offsite/?url={}", txt.get()
								);
								_ = window().open_with_url(&share_url);
							}
						>
							<Icon icon=LINKEDIN_LOGO size="18px"/> " LinkedIn"
						</button>
						<button
							class="btn btn-xs btn-ghost rounded-sm p-1 mx-1 flex" 
							style="color: #AE2983;"
							on:click=move |_| { 
								let share_url = format!(
									"https://www.facebook.com/sharer.php?u={}", txt.get()
								);
								_ = window().open_with_url(&share_url);
							}
						>
							<Icon icon=FACEBOOK_LOGO size="18px"/> " Facebook"
						</button>
						<button
							class="btn btn-xs btn-ghost rounded-sm p-1 mx-1 flex" 
							style="color: #AE2983;"
							on:click=move |_| { 
								let share_url = format!(
									"https://x.com/intent/tweet?text={}", txt.get()
								);
								_ = window().open_with_url(&share_url);
							}
						>
							<Icon icon=TWITTER_LOGO size="18px"/> " Twitter"
						</button>
					</div>
				</div>
			</ul>
		</details>
	}
}

const DEFAULT_STYLE: &str = "
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
	}";
