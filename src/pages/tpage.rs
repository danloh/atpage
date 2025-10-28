use itertools::Itertools;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use crate::{
	pages::atpage::RecCard, 
	resources::{
		atpage::RecordsRes, 
		auth::resolve_did, 
		service::{
			frontpage::get_frontpage_records, grain::get_grain_records, leaflet::get_leaflet_records, pinksea::get_pinksea_records, recipe::get_recipe_records, ruthub::get_ruthub_records, smokesignal::get_smokesignal_records, whitewind::get_whtwnd_records
		}
	}
};

#[component]
pub fn TestAtPage() -> impl IntoView {
	let params = use_params_map();
  let did: String = params.read().get("did").unwrap_or_default();

	let rec_res = LocalResource::new(
		move || fetch_all_records(did.clone())
	);

	view! {
		<div class="flex flex-col items-center justify-center gap-2 at-box">
			<Suspense fallback=move || "Loading">
				{move || match rec_res.get() {
					Some(res) => {
						res
							.records
							.iter()
					    .map(|rec| view! { 
								<RecCard rec=rec.clone() profile=RwSignal::new(Default::default()) /> 
							})
					    .collect_view()
							.into_any()
					}
					_ => "error".into_any()
				}}
			</Suspense>
		</div>
	}
}

/// for test only
pub async fn fetch_all_records(did: String) -> RecordsRes {
  let serv = resolve_did(&did).await.service;
  let mut vec = Vec::new();
  
  let (mut frontpages, _f_cur) = get_frontpage_records(&did, &serv, None).await;
  vec.append(&mut frontpages);

  let (mut wnds, _w_cur) = get_whtwnd_records(&did, &serv, None).await; 
  vec.append(&mut wnds);

  let (mut pinkseas, _p_cur) = get_pinksea_records(&did, &serv, None).await; 
  vec.append(&mut pinkseas);

	let (mut ruts, _r_cur) = get_ruthub_records(&did, &serv, None).await;
	vec.append(&mut ruts);

	let (mut smokes, _s_cur) = get_smokesignal_records(&did, &serv, None).await;
	vec.append(&mut smokes);

	let (mut grains, _g_cur) = get_grain_records(&did, &serv, None).await;
	vec.append(&mut grains);

	let (mut recipes, _rc_cur) = get_recipe_records(&did, &serv, None).await;
	vec.append(&mut recipes);

	let (mut leafs, _lf_cur) = get_leaflet_records(&did, &serv, None).await;
	vec.append(&mut leafs);

  let final_vec = vec
    .into_iter()
    .sorted_by(|a, b| Ord::cmp(&b.timestamp, &a.timestamp))
    .collect();

	// gloo::console::log!("all records res: ", format!("{:?}", final_vec));

  RecordsRes {
    records: final_vec,
    cursor: None,
  }
}
