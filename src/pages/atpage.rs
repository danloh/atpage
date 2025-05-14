use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use leptos_meta::*;

use crate::helper::md::md2html;
use crate::helper::utils::ts_to_dt;
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

	let txt = format!("@{} | atpage", &handle);
	let profile = LocalResource::new(
		move || get_profile(handle.clone())
	);

	view! {
		<Title text={txt} />
		<Suspense fallback=move || "Loading" >
			{move || match profile.get() {
				Some(p) => { 
					let p = p.as_ref().clone();
					match p {
						Ok(profile) => {
							view! { <AtView profile=profile.clone() /> }.into_any()
						}
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
	let atpage = LocalResource::new(
		move || fetch_atpage(p_signal.get().did)
	);

	view! {
		<Suspense fallback=move || "Loading">
			{move || match atpage.get() {
				Some(res) => {
					let res = res.as_ref().cloned().unwrap_or_default();
					view! {
						<Style>{res.style.unwrap_or_default()}</Style>
						<div class="min-h-screen w-screen at-screen">
							<div class="flex flex-col items-center justify-center p-2 mx-auto max-w-2xl at-page">
								<ProfileView profile=p_signal.get() />
								<div class="flex flex-col items-center justify-center gap-2 at-view">
									<div class="flex flex-col gap-2 w-full at-wrap">
										<div 
											class="prose flex items-center justify-center p-2 at-desc" 
											inner_html={md2html(&res.description.clone().unwrap_or_default()).html}
										></div>
										<AtBox did=p_signal.get().did services=res.services.clone() profile=p_signal /> 
									</div>
								</div>
							</div>
						</div>
					}
					.into_any()
				}
				None => "error".into_any()
			}}
		</Suspense>	
	}
}

#[component]
pub fn AtBox(
	did: String, 
	services: Vec<String>, 
	profile: RwSignal<ProfileRes>,
) -> impl IntoView {
	let rec_res = LocalResource::new(
		move || fetch_records((did.clone(), services.clone()))
	);

	view! {
		<div class="flex flex-col items-center justify-center gap-2 at-box">
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
fn RecCard(rec: AtRecord, profile: RwSignal<ProfileRes>) -> impl IntoView {
	view! { 
		<div class="w-full p-2 at-rec">
			<div class="flex items-center justify-start gap-2 at-rec-meta">
				<img 
					class="h-4 w-4 rounded at-rec-meta-ava" 
					src={profile.get().avatar.clone().unwrap_or_default()} 
					loading="lazy" 
				/>
				<span class="text-xs at-rec-meta-date">
					{ts_to_dt(rec.timestamp / 1000)}
				</span>
				<a 
					href={rec.link.clone()} 
					class="link link-hover text-xs at-rec-meta-kind"     
					target="_blank"
				>
					{format!("@{}", rec.kind)}
				</a>
			</div>
			<div class="flex items-center justify-start gap-2 at-rec-title">
				<a 
					href={rec.link} 
					target="_blank" 
					class="font-bold text-xl link link-hover at-rec-title-link"
				>
					{rec.title}
				</a>
			</div>
			<div 
				class="flex-1 prose no-scrollbar at-rec-ctn" 
				style="max-height: 240px; overflow: auto" 
				inner_html={md2html(&rec.content).html} 
			></div> 
		</div>
	}
}

#[component]
pub fn ProfileView(profile: ProfileRes) -> impl IntoView {
	view! {
		<div class="flex flex-wrap items-center justify-center gap-2 at-prf">
			<img 
				class="h-8 w-8 rounded-full at-prf-img" 
				src={profile.avatar.clone().unwrap_or_default()} 
				loading="lazy" 
			/>
			<a 
				href={format!("https://bsky.app/profile/{}", profile.handle.clone())} 
				target="_blank" 
				class="link link-hover text-2xl at-prf-handle"
			>
				{
					profile.displayName.clone().map(|n| {
						if n.trim().is_empty() { profile.handle.clone() } else { n }
					})
					.unwrap_or_else(|| profile.handle.clone())
				}
			</a>
		</div>	
	}
}
